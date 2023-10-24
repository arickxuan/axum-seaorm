
use tonic::transport::server::{ Routes};


mod user;


pub fn init_grpc() ->Routes{
    let reflection_service = tonic_reflection::server::Builder::configure()
        //.register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(user::user::add_greeter())
        //.add_service(GreeterServer::new(GrpcServiceImpl::default()))
        .into_service();





    grpc

}

