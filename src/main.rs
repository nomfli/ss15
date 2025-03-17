use bevy::prelude::*;
use bevy_renet::netcode::*;
use bevy_renet::*;

mod client;
mod server;
mod shared;

use crate::client::*;
use crate::server::*;
use crate::shared::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = (&args[1]).as_str();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_resource::<Lobby>();
    app.add_systems(Startup, startup);
    match exec_type {
        "server" => {
            app.add_plugins(RenetServerPlugin);
            app.add_plugins(NetcodeServerPlugin);
            let (server, transport) = new_renet_server();
            app.insert_resource(server);
            app.insert_resource(transport);
            app.add_systems(Update, move_players_system);

            app.add_systems(Update, velocity);
            app.add_systems(Update, server_sync_players);
            app.add_systems(Update, update_server_system);
        }

        "client" => {
            app.add_plugins(RenetClientPlugin);
            app.add_plugins(NetcodeClientPlugin);
            app.init_resource::<PlayerInput>();
            let (client, transport) = new_renet_client();
            app.insert_resource(client);
            app.insert_resource(transport);
            app.add_systems(Update, player_input);
            app.add_systems(Update, client_send_input);
            app.add_systems(Update, client_sync_players);
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
