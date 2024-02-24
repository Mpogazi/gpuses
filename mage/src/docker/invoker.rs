use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::image::ListImagesOptions;
use bollard::models::ImageSummary;
use bollard::errors::Error;

use rand::Rng;
use std::default::Default;

pub struct Invoker {
    docker: Docker
}

impl Invoker {
    pub fn new() -> Invoker {
        Invoker {
            docker: Docker::connect_with_local_defaults().unwrap()
        }
    }

    pub async fn images(&self) -> Result<Vec<ImageSummary>, Error>{
        let images = self.docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await.unwrap();

        for image in &images {
            println!("image: {:?}", image);
        };
        Ok(images)
    }

    pub async fn run_container(&self) -> Result<&'static str, Error>{
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