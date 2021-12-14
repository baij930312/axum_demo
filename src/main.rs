use core::panic;
use std::collections::HashMap;

use tokio::net::{TcpListener, TcpStream};

use mini_redis::{Command, Connection, Frame};
use tokio;

#[tokio::main]
async fn main() {
    let tcp = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = tcp.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(stream: TcpStream) {
    let mut conn = Connection::new(stream);
    let mut db = HashMap::new();
    while let Some(frame) = conn.read_frame().await.unwrap() {
        let res = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                println!("{:?}", cmd);
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("{:?}", db);
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("no {:?}", cmd),
        };
        conn.write_frame(&res).await.unwrap();
    }
    // if  {
    // };
}
