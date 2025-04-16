use bevy::prelude::*;

mod client;
mod server;
mod shared;


use crate::{
    client::{
        network::{init::ClientInitPlug, receive::ClientNetworkPlug, sending::ClientSendingPlug},
        render::{
            connection::ConnectionPlug, init::InitRenderPlug, input::InputClientPlug,
            movement::MovementClientPlug,
        },
    },
    server::{
        logic::{init::ServerInitPlug, movement::MovementServerPlug},
        network::{
            connection::ConnectionHandlerPlug, init::StartupServerPlug, sending::ServerSendPlug,
            update_server_system::UpdateServerPlug,
        },
    },
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
                ConnectionHandlerPlug,
                StartupServerPlug,
                ServerSendPlug,
                UpdateServerPlug,
            ));
        }

        "client" => {
            app.add_plugins((
                ClientInitPlug,
                ClientNetworkPlug,
                ClientSendingPlug,
                ConnectionPlug,
                InitRenderPlug,
                InputClientPlug,
                MovementClientPlug,

            ));
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
