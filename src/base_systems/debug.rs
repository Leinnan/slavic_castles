use std::fmt;

use crate::data::consts::*;
use bevy::app::{App, Plugin};
// use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{
    bevy_inspector::hierarchy::SelectedEntities, DefaultInspectorConfigPlugin,
};
// use iyes_perf_ui::prelude::*;

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub enum PanelDisplay {
    #[default]
    Hierarchy,
    Resources,
    Assets,
    Inspector,
}

impl PanelDisplay {
    pub const VALUES: [Self; 4] = [
        Self::Hierarchy,
        Self::Resources,
        Self::Assets,
        Self::Inspector,
    ];
}

impl fmt::Display for PanelDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Resource)]
pub struct Inspector {
    pub left_panel: PanelDisplay,
    pub right_panel: PanelDisplay,
    pub active: bool,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            left_panel: PanelDisplay::Hierarchy,
            right_panel: PanelDisplay::Inspector,
            active: false,
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_egui);
        app.add_systems(
            Update,
            inspector_ui, //.run_if(not(in_state(crate::states::MainState::Editor))),
        )
        // .add_systems(
        //     Update,
        //     (toggle_perf_ui).run_if(input_just_pressed(KeyCode::F2)),
        // )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        // .add_plugins(PerfUiPlugin)
        // .insert_resource(bevy_mod_picking::debug::DebugPickingMode::Normal)
        .add_plugins((EguiPlugin, DefaultInspectorConfigPlugin));
    }
}
// fn toggle_perf_ui(q: Query<Entity, With<PerfUiRoot>>, mut commands: Commands) {
//     let is_empty = q.is_empty();
//     if !is_empty {
//         for e in q.iter() {
//             commands.entity(e).despawn_recursive();
//         }
//         return;
//     }
//     // create a simple Perf UI with default settings
//     // and all entries provided by the crate:
//     commands.spawn((
//         PerfUiRoot::default(),
//         // when we have lots of entries, we have to group them
//         // into tuples, because of Bevy Rust syntax limitations
//         (
//             PerfUiEntryFPS::default(),
//             PerfUiEntryFPSWorst::default(),
//             PerfUiEntryFrameTime::default(),
//             PerfUiEntryFrameTimeWorst::default(),
//             PerfUiEntryFrameCount::default(),
//             PerfUiEntryEntityCount::default(),
//             PerfUiEntryCpuUsage::default(),
//             PerfUiEntryMemUsage::default(),
//         ),
//         (
//             PerfUiEntryFixedTimeStep::default(),
//             PerfUiEntryFixedOverstep::default(),
//             PerfUiEntryRunningTime::default(),
//             PerfUiEntryClock::default(),
//         ),
//         (
//             PerfUiEntryCursorPosition::default(),
//             PerfUiEntryWindowResolution::default(),
//             PerfUiEntryWindowMode::default(),
//             PerfUiEntryWindowPresentMode::default(),
//         ),
//     ));
// }

fn inspector_ui(
    world: &mut World,
    mut selected_entities: Local<SelectedEntities>,
    mut inspector_data: Local<Inspector>,
) {
    use bevy::window::PrimaryWindow;
    if world
        .get_resource::<ButtonInput<KeyCode>>()
        .unwrap()
        .just_released(KeyCode::F1)
    {
        inspector_data.active = !inspector_data.active;
    }
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();
    egui::SidePanel::left("left")
        .default_width(250.0)
        .show_animated(egui_context.get_mut(), inspector_data.active, |ui| {
            ui.horizontal(|ui| {
                for value in PanelDisplay::VALUES {
                    ui.selectable_value(&mut inspector_data.left_panel, value, value.to_string());
                }
            });
            ui.separator();
            ui.label(egui::RichText::new("Press F1 to toggle UI").small());
            ui.label(egui::RichText::new(format!("{} ( {} )", GIT_DATE, GIT_HASH)).small());
            ui.add_space(15.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                match &inspector_data.left_panel {
                    PanelDisplay::Hierarchy => {
                        bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                            world,
                            ui,
                            &mut selected_entities,
                        );
                    }
                    PanelDisplay::Resources => {
                        bevy_inspector_egui::bevy_inspector::ui_for_resources(world, ui);
                    }
                    PanelDisplay::Inspector => match selected_entities.as_slice() {
                        &[entity] => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                        }
                        entities => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                                world, entities, ui,
                            );
                        }
                    },
                    PanelDisplay::Assets => {
                        bevy_inspector_egui::bevy_inspector::ui_for_all_assets(world, ui);
                    }
                }

                ui.allocate_space(ui.available_size());
                ui.add_space(10.0);
            });
        });

    egui::SidePanel::right("right")
        .default_width(250.0)
        .show_animated(egui_context.get_mut(), inspector_data.active, |ui| {
            ui.horizontal(|ui| {
                for value in PanelDisplay::VALUES {
                    ui.selectable_value(&mut inspector_data.right_panel, value, value.to_string());
                }
            });
            ui.separator();
            ui.add_space(10.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                match &inspector_data.right_panel {
                    PanelDisplay::Hierarchy => {
                        bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                            world,
                            ui,
                            &mut selected_entities,
                        );
                    }
                    PanelDisplay::Resources => {
                        bevy_inspector_egui::bevy_inspector::ui_for_resources(world, ui);
                    }
                    PanelDisplay::Inspector => match selected_entities.as_slice() {
                        &[entity] => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                        }
                        entities => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                                world, entities, ui,
                            );
                        }
                    },
                    PanelDisplay::Assets => {
                        bevy_inspector_egui::bevy_inspector::ui_for_all_assets(world, ui);
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });
}

fn configure_egui(mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts) {
    #[cfg(any(windows, target_os = "macos"))]
    {
        if let Some((regular, semibold)) = get_fonts() {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "regular".to_owned(),
                egui::FontData::from_owned(regular).into(),
            );
            fonts.font_data.insert(
                "semibold".to_owned(),
                egui::FontData::from_owned(semibold).into(),
            );

            // Put my font first (highest priority) for proportional text:
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "regular".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Name("semibold".into()))
                .or_default()
                .insert(0, "semibold".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("regular".to_owned());

            // Tell egui to use these fonts:
            contexts.ctx_mut().set_fonts(fonts);
        }
    }
    contexts.ctx_mut().style_mut(|style| {
        for font_id in style.text_styles.values_mut() {
            font_id.size *= 1.2;
        }
    });
}

#[cfg(target_os = "macos")]
fn get_fonts() -> Option<(Vec<u8>, Vec<u8>)> {
    use std::fs;
    let font_path = std::path::Path::new("/System/Library/Fonts");

    let regular = fs::read(font_path.join("SFNSRounded.ttf")).ok()?;
    let semibold = fs::read(font_path.join("SFCompact.ttf")).ok()?;

    Some((regular, semibold))
}

#[cfg(windows)]
fn get_fonts() -> Option<(Vec<u8>, Vec<u8>)> {
    use std::fs;

    let app_data = std::env::var("APPDATA").ok()?;
    let font_path = std::path::Path::new(&app_data);

    let regular = fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos.ttf")).ok()?;
    let semibold =
        fs::read(font_path.join("../Local/Microsoft/Windows/Fonts/aptos-bold.ttf")).ok()?;

    Some((regular, semibold))
}
