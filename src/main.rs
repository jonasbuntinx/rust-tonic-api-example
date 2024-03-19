use tonic::{transport::Server, Request, Response, Status};

use proto::hello_world_server::{HelloWorld, HelloWorldServer};
use proto::{HelloReply, HelloRequest};

mod proto {
    tonic::include_proto!("api");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("api_descriptor");
}

#[derive(Debug, Default)]
struct HelloWorldService {
    greeting: String,
}

#[tonic::async_trait]
impl HelloWorld for HelloWorldService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let reply = HelloReply {
            message: format!("{} {}!", &self.greeting, input.name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let hello_world = HelloWorldService {
        greeting: String::from("Hello"),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(HelloWorldServer::new(hello_world))
        .serve(addr)
        .await?;

    Ok(())
}
