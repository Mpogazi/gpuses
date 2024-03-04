use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::models::PortBinding;
use bollard::secret::HostConfig;
use bollard::Docker;

use std::collections::HashMap;
use std::process::Command;

use rand::Rng;

#[derive(Debug)]
pub struct ContainerProperties {
    pub id: String,
    pub name: String,
    pub is_running: bool,
    pub created: bool,
    pub started: bool,
}

pub struct Images {
    docker: Docker,
}

impl Images {
    pub fn new() -> Images {
        Images {
            docker: Docker::connect_with_local_defaults().unwrap(),
        }
    }

    async fn image_exists(&self, image: &str) -> bool {
        match self.docker.inspect_image(image).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn build_image(&self) {
        let path = "src/config/Dockerfile";
        let image = "kowry_ubuntu:latest";

        if self.image_exists(image).await {
            println!("Image already exists!");
        } else {
            let status = Command::new("docker")
                .arg("build")
                .arg("-t")
                .arg(image)
                .arg("-f")
                .arg(path)
                .arg(".")
                .status()
                .expect("Failed to start Docker build process");

            if status.success() {
                println!("Docker image built successfully!");
            } else {
                eprintln!("Failed to build Docker image");
            }
        }
    }

    pub async fn start_image(
        &self,
        _memory: f32,
        _cpu: f32,
        _gpu: f32,
    ) -> Result<ContainerProperties, Error> {
        self.build_image().await;
        let config = Config {
            image: Some("kowry_ubuntu"),
            cmd: Some(vec![]),
            host_config: Some(HostConfig {
                port_bindings: Some(HashMap::from([(
                    String::from("22/tcp"),
                    Some(vec![PortBinding {
                        host_ip: Some(String::from("127.0.0.1")),
                        host_port: Some(String::from("2222")),
                    }]),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        };

        let container_options = CreateContainerOptions {
            name: format!("container_{}", rand::thread_rng().gen_range(0..10000)),
            platform: Some(format!("arm64")),
        };

        let container = self
            .docker
            .create_container(Some(container_options.clone()), config)
            .await
            .expect("Failed to create container");

        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .unwrap();

        let container_properties: ContainerProperties = ContainerProperties {
            id: container.id.clone(),
            name: container_options.name.clone(),
            is_running: true,
            created: true,
            started: true,
        };
        Ok(container_properties)
    }
}
