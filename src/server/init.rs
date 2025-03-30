use crate::shared::{
    components::{Grabbable, PROTOCOL_ID, SERVER_ADDR},
    sprites::{SpriteName, Sprites},
};
use bevy::prelude::*;
use bevy_renet::netcode::*;
use bevy_renet::renet::*;
use bevy_renet::RenetServerPlugin;
use std::net::UdpSocket;
use std::time::SystemTime;

pub struct ServerInitPlug;

impl Plugin for ServerInitPlug {
    fn build(&self, app: &mut App) {
        let (server, transport) = new_renet_server();
        app.insert_resource(server);
        app.insert_resource(transport);
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
        app.add_systems(Startup, init);
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

pub(crate) fn init(sprites: Res<Sprites>, mut commands: Commands) {
    let name = "blue_sqr".to_string();
    let Some(sprite) = sprites.0.get(&name) else {
        panic!()
    };
    commands
        .spawn(sprite.clone())
        .insert(SpriteName(name))
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Grabbable(true));
}
