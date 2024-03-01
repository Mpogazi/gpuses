use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::secret::HostConfig;
use bollard::Docker;

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
        memory: f32,
        cpu: f32,
        gpu: f32,
    ) -> Result<ContainerProperties, Error> {
        let config = Config {
            image: Some("ubuntu"),
            cmd: Some(vec!["/bin/bash"]),
            host_config: Some(HostConfig {
                port_bindings: Some(
                    [(
                        "22/tcp".to_string(),
                        Some(vec![("127.0.0.1".to_string(), "2222".to_string())]),
                    )]
                    .iter()
                    .cloned()
                    .collect(),
                ),
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
            .create_container(Some(container_options), config)
            .await
            .expect("Failed to create container");

        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .expect("Failed to start container");

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
