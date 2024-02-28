use tonic::{transport::Server, Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, MultiplyRequest, MultiplyReply};
use tokio::sync::mpsc;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = hello_world::HelloReply {
            message: format!("Hello {} {}!", req.name, req.subfix),
        };

        Ok(Response::new(reply))
    }

    async fn multiply(
        &self,
        request: Request<MultiplyRequest>,
    ) -> Result<Response<MultiplyReply>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let reply = hello_world::MultiplyReply {
            message: format!("Result {}!", req.number1 * req.number2),
        };

        Ok(Response::new(reply))
    }


        type StreamHelloStream =  tokio_stream::wrappers::ReceiverStream<Result<HelloReply, Status>>;
        async fn stream_hello(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<Self::StreamHelloStream>, Status> {
               let req = request.into_inner();
               let ( tx, rx) = mpsc::channel(4);
                        tokio::spawn(async move {
                            for _ in 0..10{
                                let _ =  tx.send(Ok(HelloReply {
                                    message: format!("hello {}", req.name),
                                }))
                                .await;
                            }
                        });
                        Ok(Response::new(
                              tokio_stream::wrappers::ReceiverStream::new(rx)
                          ))
        }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
