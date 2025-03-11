use core::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000);
pub const PROTOCOL_ID: u64 = 7;
pub const PLAYER_MOVE_SPEED: f32 = 1.0;
