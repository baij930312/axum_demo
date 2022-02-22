use std::{thread::sleep, time::Duration};

use tokio::{select, sync::mpsc, sync::oneshot};

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(val) => val,
        None => return None,
    };
    Some(i.to_string())
}

//  #[tokio::main]
// async fn main() {
//     let (tx1, mut rx1) = mpsc::channel(128);

//     let mut done = false;
//     let operation = action(None);
//     tokio::pin!(operation);
//     tokio::spawn(async move {
//         let _ = tx1.send(1).await;
//         let _ = tx1.send(4).await;
//         let _ = tx1.send(3).await;
//         let _ = tx1.send(2).await;
//     });
//     loop {
//         select! {
//             res = &mut operation ,if !done=>{
//                 done = true;
//                 if let Some(val) = res {
//                     println!("{}",val );
//                     return;
//                 }

//             }
//             Some(num) = rx1.recv()=>{
//                 if num % 2 ==0{
//                     operation.set(action(Some(num)));
//                     done  = false;
//                 }
//             }
//         }
//     }
// }

async fn asdsadsa() -> i32 {
    1
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    let tx3 = tx1.clone();
    let handle1 = tokio::spawn(async move {
        select! {
            val = asdsadsa() =>{
                let res = tx3.send(val).await.unwrap();
                println!("{:?}", res );
            }
            _ = tx3.closed() =>{
                println!("123213123" );
            }
        };
    });

    let handle2 = tokio::spawn(async move {
        let _ = tx2.send("two").await.unwrap();
    });

    let handle3 = tokio::spawn(async move {
        loop {
            select! {
                Some(msg) = rx1.recv() =>{
                    println!("111111 " );
                    println!("{:?}",msg );
                },
                Some(msg)  = rx2.recv() =>{
                    println!("222222 " );
                    println!("{:?}",msg );
                },
                else => break,
            }
        }
    });

    sleep(Duration::from_secs(2));
}
