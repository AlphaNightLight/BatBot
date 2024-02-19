use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::ble_connector::BleStatistics;

#[derive(Component)]
struct Fps;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(
        // Here we are able to call the `From` method instead of creating a new `TextSection`.
        // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
        (TextBundle::from_sections([
            TextSection::new("Fps: ", TextStyle::default()),
            TextSection::new("0.", TextStyle::default()),
            TextSection::new(" Blocks: ", TextStyle::default()),
            TextSection::new("0.", TextStyle::default()),
            TextSection::new(" Positions: ", TextStyle::default()),
            TextSection::new("0.", TextStyle::default()),
        ]), Fps)
    );
}
fn text_update_system(
    mut query: Query<&mut Text, With<Fps>>,
    ble_stats: Res<BleStatistics>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in &mut query {

        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
        text.sections[3].value = format!("{:.2} {:.2}/s", ble_stats.n_blocks, ble_stats.n_blocks as f32/ble_stats.dur_blocks.as_secs_f32());
        text.sections[5].value = format!("{:.2} {:.2}/s", ble_stats.n_pos, ble_stats.n_pos as f32/ble_stats.dur_pos.as_secs_f32());
    }
}
pub struct StatisticsPlugin;

impl Plugin for StatisticsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, text_update_system);
    }
}