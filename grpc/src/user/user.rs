
use tonic::{Response as TonicResponse, Status};
use tonic::transport::server::Router;

mod pb {
    include!("../../../proto/helloworld.rs");

    //pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("helloworld_descriptor");
}

pub use pb::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

pub fn add_greeter() -> GreeterServer<GrpcServiceImpl> {
    //router.add_service()
    GreeterServer::new(GrpcServiceImpl::default())
}

#[derive(Default)]
pub struct GrpcServiceImpl {}

#[tonic::async_trait]
impl Greeter for GrpcServiceImpl {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<TonicResponse<HelloReply>, Status> {
        //tracing::info!("Got a request from {:?}", request.remote_addr());
        //println!("lsi le {}",&request.into_inner().name);

        let reply = HelloReply {
            message: format!("Hello 123 {}!", &request.into_inner().name),
        };

        Ok(TonicResponse::new(reply))
    }
}
