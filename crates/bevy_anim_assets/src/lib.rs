use std::any::TypeId;

use bevy_animation::{
    AnimationClip, AnimationEntityMut, AnimationEvaluationError, AnimationEvent, AnimationTargetId,
    animated_field,
    animation_curves::{
        AnimatableCurve, AnimatableKeyframeCurve, AnimatableProperty, AnimatedField, EvaluatorId,
    },
    graph::AnimationGraph,
};
use bevy_asset::{Asset, AssetApp, AsyncReadExt, io::Reader};
use bevy_color::{Color, Srgba};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::name::Name;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_text::TextColor;
use bevy_ui::{Node, UiTransform, Val};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, Reflect)]
pub enum CurveValueType {
    #[default]
    UiTransformScale,
    TextColor,
    UiNodeTopPosition,
}

#[derive(Clone)]
pub struct UiNodeTopPositionProperty;

impl AnimatableProperty for UiNodeTopPositionProperty {
    type Property = f32;

    fn evaluator_id(&'_ self) -> EvaluatorId<'_> {
        EvaluatorId::Type(TypeId::of::<UiNodeTopPositionProperty>())
    }

    fn get_mut<'a>(
        &'_ self,
        entity: &'a mut AnimationEntityMut,
    ) -> Result<&'a mut Self::Property, AnimationEvaluationError> {
        let node = entity
            .get_mut::<Node>()
            .ok_or(AnimationEvaluationError::ComponentNotPresent(TypeId::of::<
                Node,
            >(
            )))?
            .into_inner();
        match node.top {
            Val::Px(ref mut p) => Ok(p),
            Val::Percent(ref mut p) => Ok(p),
            _ => Err(
                AnimationEvaluationError::InconsistentEvaluatorImplementation(TypeId::of::<Node>()),
            ),
        }
    }
}
// A type that represents the color of the first text section.
//
// We implement `AnimatableProperty` on this to define custom property accessor logic
#[derive(Clone)]
pub struct TextColorProperty;

impl AnimatableProperty for TextColorProperty {
    type Property = Srgba;

    fn evaluator_id(&'_ self) -> EvaluatorId<'_> {
        EvaluatorId::Type(TypeId::of::<TextColorProperty>())
    }

    fn get_mut<'a>(
        &'_ self,
        entity: &'a mut AnimationEntityMut,
    ) -> Result<&'a mut Self::Property, AnimationEvaluationError> {
        let text_color = entity
            .get_mut::<TextColor>()
            .ok_or(AnimationEvaluationError::ComponentNotPresent(TypeId::of::<
                TextColor,
            >(
            )))?
            .into_inner();
        match text_color.0 {
            Color::Srgba(ref mut color) => Ok(color),
            _ => Err(AnimationEvaluationError::PropertyNotPresent(TypeId::of::<
                Srgba,
            >(
            ))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Reflect)]
pub struct CurveData {
    pub target_name: String,
    pub curve_type: CurveValueType,
    #[reflect(ignore)]
    pub values: Vec<(f32, Value)>,
}

#[derive(AnimationEvent, Reflect, Clone)]
pub struct AnimationBaseEvent;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Reflect)]
pub struct ClipEventData(pub f32);

#[derive(Debug, Serialize, Deserialize, Clone, Default, Asset, Reflect)]
pub struct AnimationDataClip {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub name: String,
    pub curves: Vec<CurveData>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub events: Vec<ClipEventData>,
}

impl From<&AnimationDataClip> for AnimationClip {
    fn from(asset: &AnimationDataClip) -> Self {
        let mut clip = AnimationClip::default();
        for curve in &asset.curves {
            curve.add_animatable_curve(&mut clip);
        }
        for ev in &asset.events {
            clip.add_event(ev.0, AnimationBaseEvent);
        }
        clip
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Asset, Reflect, Deref, DerefMut)]
pub struct AnimationDataAsset(pub Vec<AnimationDataClip>);

impl CurveData {
    pub fn add_animatable_curve(&self, target: &mut AnimationClip) {
        let target_name = Name::new(self.target_name.clone());
        let target_id = AnimationTargetId::from_name(&target_name);
        match self.curve_type {
            CurveValueType::UiTransformScale => {
                target.add_curve_to_target(
                    target_id,
                    AnimatableCurve::new(
                        animated_field!(UiTransform::scale),
                        AnimatableKeyframeCurve::new(self.values.iter().map(|(time, value)| {
                            (
                                *time,
                                Vec2::splat(value.as_f64().unwrap_or_default() as f32),
                            )
                        }))
                        .expect("msg"),
                    ),
                );
            }
            CurveValueType::UiNodeTopPosition => {
                target.add_curve_to_target(
                    target_id,
                    AnimatableCurve::new(
                        UiNodeTopPositionProperty,
                        AnimatableKeyframeCurve::new(self.values.iter().map(|(time, value)| {
                            (*time, value.as_f64().unwrap_or_default() as f32)
                        }))
                        .expect("msg"),
                    ),
                );
            }
            CurveValueType::TextColor => {
                target.add_curve_to_target(
                    target_id,
                    AnimatableCurve::new(
                        TextColorProperty,
                        AnimatableKeyframeCurve::new(self.values.iter().map(|(time, value)| {
                            (*time, {
                                if let Some(color) = value.as_str().and_then(|s| Srgba::hex(s).ok())
                                {
                                    color
                                } else {
                                    serde_json::from_value(value.clone())
                                        .map_or_else(|_| Srgba::GREEN, |e| e)
                                }
                            })
                        }))
                        .expect("msg"),
                    ),
                );
            }
        }
    }
}

pub struct AnimationLoaderPlugin;

impl bevy_app::Plugin for AnimationLoaderPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset_loader::<AnimationsAssetLoader>();
    }
}

#[derive(Default)]
pub struct AnimationsAssetLoader;

impl bevy_asset::AssetLoader for AnimationsAssetLoader {
    type Asset = bevy_animation::graph::AnimationGraph;
    type Settings = ();
    type Error = String;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = String::new();
        let Ok(_) = reader.read_to_string(&mut bytes).await else {
            return Err("Failed to read the animation json file".to_string());
        };
        let Ok(data) = serde_json::from_str::<AnimationDataAsset>(&bytes) else {
            return Err("Failed to parse the animation json file".to_string());
        };
        let mut clips = vec![];
        for (index, clip) in data.iter().enumerate() {
            let clip_data: AnimationClip = clip.into();
            let label = if clip.name.is_empty() {
                format!("clip_{}", index)
            } else {
                clip.name.clone()
            };
            let clip_handle = load_context.add_labeled_asset(label, clip_data);
            clips.push(clip_handle);
        }
        let (graph, _) = AnimationGraph::from_clips(clips);
        Ok(graph)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["anim.json"];
        EXTENSIONS
    }
}
