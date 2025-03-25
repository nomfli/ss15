use bevy::prelude::*;

mod client;
mod server;
mod shared;

use crate::client::{
    hands::HandsClientPlug, init::ClientInitPlug, input::InputClientPlug,
    movement::MovementClientPlug, sync_players::ClientSyncPlayersPlug,
};
use crate::server::{
    hands::HandsServerPlug, init::ServerInitPlug, movement::MovementServerPlug,
    update_server_system::UpdateServerPlug,
};
use crate::shared::resource::ResInitPlug;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = (&args[1]).as_str();
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ResInitPlug));

    match exec_type {
        "server" => {
            app.add_plugins((
                HandsServerPlug,
                ServerInitPlug,
                MovementServerPlug,
                UpdateServerPlug,
            ));
        }

        "client" => {
            app.add_plugins((
                ClientInitPlug,
                InputClientPlug,
                HandsClientPlug,
                MovementClientPlug,
                ClientSyncPlayersPlug,
            ));
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
