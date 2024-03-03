use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::secret::HostConfig;
use bollard::Docker;
use bollard::models::PortBinding;

use std::collections::HashMap;

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

    pub async fn start_image(
        &self,
        _memory: f32,
        _cpu: f32,
        _gpu: f32,
    ) -> Result<ContainerProperties, Error> {
        let config = Config {
            image: Some("ubuntu"),
            cmd: Some(vec!["/bin/bash", "-c", "apt-get update && apt-get install -y openssh-server && service ssh start && sleep infinity"]),
            host_config: Some(HostConfig {
                port_bindings: Some(HashMap::from([
                    (String::from("22/tcp"), Some(vec![PortBinding {
                        host_ip: Some(String::from("127.0.0.1")),
                        host_port: Some(String::from("2222")),
                    }])),
                ])),
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

        self.docker.start_container(&container.id, None::<StartContainerOptions<String>>).await.unwrap();

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