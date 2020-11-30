mod codec;
use crate::codec::MQTTCodec;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{error::Error, println};

use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Framed};

use futures::{SinkExt, StreamExt};
use mqttrs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let world = async {
        println!("world");
    };
    print!("Hello, ");
    world.await;

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1883);
    let mut listener = TcpListener::bind(address).await?;

    loop {
        let (stream, address) = listener.accept().await.unwrap();
        println!("New connection : {}", address);
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }

    Ok(())
}
async fn handle_client(stream: TcpStream) {
    let mut framed = Framed::new(stream, MQTTCodec::new());

    let connect = match framed.next().await {
        Some(Ok(Connect(packet))) => {
            framed.send(ConnectAck {
                session_present: false,
                return_code: ConnectionAccepted,
            });
            packet
        }
        _ => {
            println!("Did not receive packet");
            return;
        }
    };
}
