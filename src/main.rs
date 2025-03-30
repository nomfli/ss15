use bevy::prelude::*;

mod client;
mod server;
mod shared;

use crate::client::{
    init::ClientInitPlug, input::InputClientPlug, movement::MovementClientPlug,
    network::ClientSyncPlayersPlug,
};
use crate::server::{
    connection::ConnectionHandlerPlug, init::ServerInitPlug, movement::MovementServerPlug,
    update_server_system::UpdateServerPlug,
};
use crate::shared::{resource::ResInitPlug, sprites::SpritesPlug};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = args[1].as_str();
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, ResInitPlug, SpritesPlug));

    match exec_type {
        "server" => {
            app.add_plugins((
                ServerInitPlug,
                MovementServerPlug,
                UpdateServerPlug,
                ConnectionHandlerPlug,
            ));
        }

        "client" => {
            app.add_plugins((
                ClientInitPlug,
                InputClientPlug,
                MovementClientPlug,
                ClientSyncPlayersPlug,
            ));
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
