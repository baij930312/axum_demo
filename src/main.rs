use std::net::SocketAddr;

use demo::hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloRequest, HelloResponse,
};
use tokio::runtime::Builder;
use tonic::{transport::Server, Request, Response, Status};

struct MyGreetter {}

#[tonic::async_trait]
impl Greeter for MyGreetter {
    async fn say_hello(
        &self,
        req: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        Ok(Response::new(HelloResponse {
            greeting: format!("hello,{}!", req.get_ref().name),
        }))
    }
}

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let addr: SocketAddr = "127.0.0.1:5001".parse().unwrap();
        let greeter = MyGreetter {};

        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await
            .unwrap();
    })
}
