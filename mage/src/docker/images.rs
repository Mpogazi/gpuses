use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::errors::Error;

use rand::Rng;


pub struct Images {
    docker: Docker
}

impl Images {
    pub fn new() -> Images {
        Images {
            docker: Docker::connect_with_local_defaults().unwrap()
        }
    }

    pub async fn start_image(&self) -> Result<&'static str, Error>{
        let container_name = format!("container_{}", rand::thread_rng().gen_range(0..10000));

        let config = Config {
            image: Some("ubuntu"),
            cmd: Some(vec!["/bin/bash"]),
            ..Default::default()
        };

        let container = CreateContainerOptions {
            name: container_name.clone(),
            platform: Some(format!("arm64")),
        };

        let start_options = StartContainerOptions {
            detach_keys: "ctrl-p"
        };

        let _ = self.docker.create_container(Some(container), config).await.unwrap();
        let _ = self.docker.start_container(&container_name, Some(start_options)).await.unwrap();

        self.docker.stop_container(&container_name, Some(StopContainerOptions {
            t: 3600,
        })).await.unwrap();

        println!("Container started");
        Ok("Image pulled and container started successfully!")
    }
}