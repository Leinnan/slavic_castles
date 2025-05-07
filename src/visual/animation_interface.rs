use bevy::{
    animation::{AnimationEntityMut, AnimationEvaluationError, AnimationTargetId, animated_field},
    color,
    prelude::*,
};
use std::any::TypeId;
use std::borrow::Cow;

#[derive(Clone, Debug, Reflect)]
// Holds information about the animation we programmatically create.
pub struct AnimationInfo {
    // The name of the animation target (in this case, the text).
    pub target_name: Name,
    // The ID of the animation target, derived from the name.
    pub target_id: AnimationTargetId,
    // The animation graph asset.
    pub graph: Handle<AnimationGraph>,
    // The index of the node within that graph.
    pub pos_node_index: AnimationNodeIndex,
    // The index of the node within that graph.
    pub neg_node_index: AnimationNodeIndex,
}

impl AnimationInfo {
    // Programmatically creates the UI animation.
    pub fn create(
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
        name: impl Into<Cow<'static, str>>,
    ) -> AnimationInfo {
        // Create an ID that identifies the text node we're going to animate.
        let animation_target_name = Name::new(name);
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
        let mut clips = vec![];

        for color in [
            color::palettes::tailwind::GREEN_400,
            color::palettes::tailwind::RED_500,
        ] {
            // Allocate an animation clip.
            let mut animation_clip = AnimationClip::default();

            // Create a curve that animates font size.
            animation_clip.add_curve_to_target(
                animation_target_id,
                AnimatableCurve::new(
                    animated_field!(Transform::scale),
                    AnimatableKeyframeCurve::new(
                        [0.0, 0.2, 0.6, 0.8]
                            .into_iter()
                            .zip([Vec3::splat(1.0),Vec3::splat(0.3),Vec3::splat(0.3), Vec3::splat(1.0)]),
                    )
                        .expect(
                            "should be able to build translation curve because we pass in valid samples",
                        ),
                ),
            );
            let base = color::palettes::tailwind::INDIGO_100;
            animation_clip.add_curve_to_target(
                animation_target_id,
                AnimatableCurve::new(
                    TextColorProperty,
                    AnimatableKeyframeCurve::new([0.0, 0.1, 0.5, 0.8].into_iter().zip([
                        base,
                        color,
                        color,
                        base,
                    ]))
                        .expect(
                            "should be able to build translation curve because we pass in valid samples",
                        ),
                ),
            );

            // Save our animation clip as an asset.
            let animation_clip_handle = animation_clips.add(animation_clip);
            clips.push(animation_clip_handle);
        }

        // Create an animation graph with that clip.
        let (animation_graph, animation_node_index) = AnimationGraph::from_clips(clips);
        let animation_graph_handle = animation_graphs.add(animation_graph);

        AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph_handle,
            pos_node_index: animation_node_index[0],
            neg_node_index: animation_node_index[1],
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

    fn evaluator_id(&self) -> EvaluatorId {
        EvaluatorId::Type(TypeId::of::<Self>())
    }

    fn get_mut<'a>(
        &self,
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
