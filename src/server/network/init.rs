use crate::shared::components::{PROTOCOL_ID, SERVER_ADDR};
use bevy::prelude::*;
use bevy_renet::{netcode::*, renet::*, *};
use std::{net::UdpSocket, time::SystemTime};

pub(crate) struct StartupServerPlug;

impl Plugin for StartupServerPlug {
    fn build(&self, app: &mut App) {
        let (server, transport) = new_renet_server();
        app.insert_resource(server);
        app.insert_resource(transport);
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
    }
}

pub(crate) fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());
    (server, transport)
}
