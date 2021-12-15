use std::{thread::sleep, time::Duration};

use bytes::Bytes;
use mini_redis::client;
use tokio::{
    self,
    sync::{mpsc, oneshot},
};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        respender: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        respender: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, respender } => {
                    let res = client.get(&key).await.unwrap();
                    let _ = respender.send(Ok(res));
                }
                Set {
                    key,
                    value,
                    respender,
                } => {
                    let res = client.set(&key, value).await.unwrap();
                    let _ = respender.send(Ok(res));
                }
            }
        }
    });

    let tx1 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "hello".into(),
            respender: resp_tx,
        };
        sleep(Duration::from_secs(1));
        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("{:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "hello".into(),
            value: "world".into(),
            respender: resp_tx,
        };
        tx1.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("{:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

//报错 client不能同时再两个spawn中使用
//如果使用锁会导致性能损耗
// #[tokio::main]
// async fn main (){
//     let mut client = client::connect("127.0.0.1:6379").await.unwrap();
//     let t1 = tokio::spawn(async  {
//         client.get("hello").await;
//     });
//      let t2 = tokio::spawn(async  {
//         client.set("hello","world".into()).await;
//     });

//     t1.await;
//     t2.await;
// }

//channel
