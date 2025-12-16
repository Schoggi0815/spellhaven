use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};
use networking::{
    create_world::CreateWorld, start_self_session::StartSelfSession,
    start_steam_server::StartSteamServer,
    start_websocket_client::StartWebsocketClient,
    start_websocket_server::StartWebsocketServer,
};

use crate::{
    main_menu_data::MainMenuData,
    main_menu_state::{MainMenuState, hide_main_menu},
};

#[derive(Default)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(OnEnter(MainMenuState::Shown), add_menu_cam)
            .add_systems(
                EguiPrimaryContextPass,
                (
                    render_main_menu.run_if(in_state(MainMenuState::Shown)),
                    render_loading_screen
                        .run_if(in_state(MainMenuState::LoadingWorldGen)),
                ),
            )
            .add_observer(hide_main_menu);
    }
}

#[derive(Component)]
pub struct MenuCamera;

fn add_menu_cam(mut commands: Commands) {
    commands.spawn((Camera2d, MenuCamera));
}

fn render_main_menu(
    mut menu_data: Local<MainMenuData>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut commands: Commands,
    mut contexts: EguiContexts,
) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("SpellHaven");

            ui.text_edit_singleline(&mut menu_data.seed);
            if ui.button("Singleplayer").clicked() {
                let mut hasher = DefaultHasher::new();
                menu_data.seed.hash(&mut hasher);
                let seed = hasher.finish();

                info!("Seed to use: {}", seed);

                commands.trigger(StartSelfSession);
                commands.trigger(CreateWorld { seed });

                menu_state.set(MainMenuState::LoadingWorldGen);
            }

            ui.add_space(10.);

            if ui.button("Host game (Steam)").clicked() {
                let mut hasher = DefaultHasher::new();
                menu_data.seed.hash(&mut hasher);
                let seed = hasher.finish();

                info!("Seed to use: {}", seed);

                commands.trigger(StartSteamServer);
                commands.trigger(CreateWorld { seed });

                menu_state.set(MainMenuState::LoadingWorldGen);
            }

            ui.add_space(10.);

            if ui.button("Host game (Server)").clicked() {
                let mut hasher = DefaultHasher::new();
                menu_data.seed.hash(&mut hasher);
                let seed = hasher.finish();

                info!("Seed to use: {}", seed);

                commands.trigger(StartWebsocketServer);
                commands.trigger(CreateWorld { seed });

                menu_state.set(MainMenuState::LoadingWorldGen);
            }

            ui.add_space(20.);

            ui.text_edit_singleline(&mut menu_data.server_ip)
                .on_hover_text("Server Ip");
            if ui.button("Join game").clicked() {
                commands.trigger(StartWebsocketClient {
                    address: menu_data.server_ip.clone(),
                });

                menu_state.set(MainMenuState::LoadingWorldGen);
            }
        });
    });

    Ok(())
}

fn render_loading_screen(mut contexts: EguiContexts) -> Result {
    egui::CentralPanel::default().show(contexts.ctx_mut()?, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Loading");
        });
    });

    Ok(())
}
