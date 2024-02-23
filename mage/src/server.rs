use tonic::{{transport::Server, Request, Response, Status}};

use spin::container_server::{Container, ContainerServer};
use spin::{ContainerRequest, ContainerResponse};

pub mod docker;
use docker::invoker::Invoker;


pub mod spin {
    tonic::include_proto!("spin");
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
    let invoker = Invoker::new();
    invoker.images().await;

    let addr = "[::1]:50051".parse()?;
    let container_service = ContainerService::default();

    Server::builder()
        .add_service(ContainerServer::new(container_service))
        .serve(addr)
        .await?;
    Ok(())
}