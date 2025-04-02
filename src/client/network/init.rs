use crate::shared::components::{PROTOCOL_ID, SERVER_ADDR};
use bevy::prelude::*;
use bevy_renet::{netcode::*, renet::*, RenetClientPlugin};
use std::{net::UdpSocket, time::SystemTime};

struct ClientInitPlug;

impl Plugin for ClientInitPlug {
    fn build(&self, app: &mut App) {
        let (client, transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(transport);
        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);
    }
}

pub(crate) fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = rand::random::<u64>();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());
    (client, transport)
}
