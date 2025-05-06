use bevy::prelude::*;

mod client;
mod server;
mod shared;

use crate::{
    client::{
        network::{init::ClientInitPlug, receive::ClientNetworkPlug, sending::ClientSendingPlug},
        render::{
            connection::ConnectionPlug, hands::HandsClientPlug, init::InitRenderPlug,
            input::InputClientPlug, map::ClientMapPlug, movement::MovementClientPlug,
        },
    },
    server::{
        logic::{hands::HandsServerPlug, init::ServerInitPlug, movement::MovementServerPlug},
        network::{
            connection::ConnectionHandlerPlug, init::StartupServerPlug, sending::ServerSendPlug,
            update_server_system::UpdateServerPlug,
        },
    },
};

use crate::shared::{
    events::SharedEvents, map::SharedMapPlug, resource::ResInitPlug, sprites::SpritesPlug,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = args[1].as_str();
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        ResInitPlug,
        SpritesPlug,
        SharedEvents,
        SharedMapPlug,
    ));

    match exec_type {
        "server" => {
            app.add_plugins((
                ServerInitPlug,
                MovementServerPlug,
                ConnectionHandlerPlug,
                StartupServerPlug,
                ServerSendPlug,
                UpdateServerPlug,
                HandsServerPlug,
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
                HandsClientPlug,
                ClientMapPlug,
            ));
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
