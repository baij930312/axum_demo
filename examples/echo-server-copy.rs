use std::vec;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// #[tokio::main]
// async fn main()->io::Result<()>{
//     let listener = TcpListener::bind("127.0.0.1::6142").await.unwrap();

//     let (  socket,_) = listener.accept().await.unwrap();
// io::split 内部使用了arc和mutex使得读写handle可以跨线程
// socket.split 只是返回两个内部的引用，由于引用已经被返回使用所有权转移所以两个handle只能在同一线程 这种方式是0开销的
//     let (mut rt,mut wt) = io::split(socket);
//        tokio::spawn(async move{
//             wt.write_all(b"hello").await?;
//             wt.write_all(b"world").await?;
//             Ok::<_, io::Error>(())

//        });
//        let mut buf = vec![0,128];

//     loop  {
//     let n = rt.read(&mut buf).await?;
//     if n == 0 {
//         break;
//     }

//     println!("{:?}",&buf[..n] );
//     }
//     Ok(())
// }

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1::6142").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (mut rt, mut wt) = socket.split();

            if io::copy(&mut rt, &mut wt).await.is_err() {
                eprintln!("failed to copy");
                return ;
            };
        });
    }
}

