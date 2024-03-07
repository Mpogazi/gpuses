use tonic::{{transport::Server, Request, Response, Status}};

use spin::container_server::{Container, ContainerServer};
use spin::{ContainerRequest, ContainerResponse};

pub mod docker;
use docker::invoker::Invoker;

pub mod spin {
    tonic::include_proto!("spin");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = 
        tonic::include_file_descriptor_set!("spin_descriptor");
}


#[derive(Debug, Default)]
pub struct ContainerService {}

#[tonic::async_trait]
impl Container for ContainerService {
    async fn spin_container(
        &self,
        request: Request<ContainerRequest>,
    ) -> Result<Response<ContainerResponse>, Status> {
        println!("Got a request: {:?}", request);

        let _req = request.into_inner();
        let container = "container_name_12";
        let portno = "3404";

        let reply = ContainerResponse {
            successful: true,
            hostname: container.to_string(),
            port: portno.to_string(),
        };

        Ok(Response::new(reply))
    }
    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut invoker = Invoker::new();
    invoker.start_image().await;

    let addr = "[::1]:50051".parse()?;

    let container_service = ContainerService::default();
    let container_service_build = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(spin::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    

    Server::builder()
        .add_service(container_service_build)
        .add_service(ContainerServer::new(container_service))
        .serve(addr)
        .await?;
    Ok(())
}