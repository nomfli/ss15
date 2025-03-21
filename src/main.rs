use bevy::prelude::*;
use bevy_renet::netcode::*;
use bevy_renet::*;

use std::collections::HashMap;

mod client;
mod server;
mod shared;

use crate::client::init::*;
use crate::client::*;
use crate::server::hands::*;
use crate::server::movement::*;
use crate::server::*;
use crate::shared::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = (&args[1]).as_str();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_resource::<Lobby>();
    app.insert_resource(Data {
        sprite: HashMap::new(),
    });
    app.add_systems(Startup, init_data);

    match exec_type {
        "server" => {
            app.add_plugins(RenetServerPlugin);
            app.add_plugins(NetcodeServerPlugin);
            let (server, transport) = new_renet_server();
            app.insert_resource(server);
            app.insert_resource(transport);
            app.add_systems(Update, move_players_system);
            app.add_systems(Update, velocity);
            app.add_systems(Update, server_send_movement);
            app.add_systems(Update, server_send_hands);
            app.add_systems(Update, update_server_system);
            //            app.add_systems(Update, throw);
            app.add_systems(Update, grabb);
            app.add_systems(Update, change_hands);
        }

        "client" => {
            app.add_plugins(RenetClientPlugin);
            app.add_plugins(NetcodeClientPlugin);
            let (client, transport) = new_renet_client();
            app.init_resource::<PlayerInput>();
            app.insert_resource(client);
            app.insert_resource(transport);
            app.add_systems(Startup, startup);
            app.add_systems(Update, player_input);
            app.add_systems(Update, client_send_input);
            app.add_systems(Update, client_handler);
            //  app.add_systems(Update, hands_client);
        }

        _ => panic!("incorrect usage"),
    }
    app.run();
}
