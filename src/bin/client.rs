use demo::hello::{greeter_client::GreeterClient, HelloRequest};
use tokio::runtime::Builder;
use tonic::Request;

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let mut cli = GreeterClient::connect("http://127.0.0.1:5001")
            .await
            .unwrap();
      
        for i in 0..100 {
            let req = Request::new(HelloRequest {
                name: "baijin111".into(),
            });
            let res = cli.say_hello(req).await.unwrap();

            println!("{:?}", res.get_ref());
        }
    })
}
