use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::models::PortBinding;
use bollard::secret::HostConfig;
use bollard::Docker;

use rand::Rng;
use std::collections::HashMap;
use std::process::Command;
use crate::auth::keys::Keys;

#[derive(Debug)]
pub struct ContainerProperties {
    pub id: String,
    pub name: String,
    pub is_running: bool,
    pub created: bool,
    pub started: bool,
    pub ssh_key: String,
}

pub struct Images {
    docker: Docker,
    key_generator: Keys,
}

impl Images {
    pub fn new() -> Images {
        Images {
            docker: Docker::connect_with_local_defaults()
                .expect("Failed to connect to the docker daemon"),
            key_generator: Keys::new(),
        }
    }

    // TODO: figure out how to immprove the versioning of the image builds
    // Because Images should change over time.
    // Also refactor the key_path logic to be less brittle.
    async fn build_image(&self, key_path: &str) {
        let docker_file = "src/config/Dockerfile";
        let image_name = "kowry_ubuntu:latest";
        let public_key = format!("SSH_KEY_PATH={}.pub", key_path);
        let build_context = ".";

        if self.image_exists(image_name).await {
            println!("Image already exists!");
        } else {
            let mut build_command = Command::new("docker");
            build_command.args([
                "build",
                "--build-arg",
                &public_key,
                "-t",
                image_name,
                "-f",
                docker_file,
                build_context,
            ]);

            match build_command.status() {
                Ok(status) => match status.success() {
                    true => println!("Docker image built successfully!"),
                    false => eprintln!("Error building docker image!"),
                },
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
            }
        }
    }

    pub async fn start_image(
        &self,
        _memory: f32,
        _cpu: f32,
        _gpu: f32,
    ) -> Result<ContainerProperties, Error> {
        let container_name = format!("container_{}", rand::thread_rng().gen_range(0..10000));
        let key_path = self.create_ssh_key(&container_name).await.unwrap();

        self.build_image(&key_path).await;

        let config = self.create_config();
        let container_options = self.create_container_options(container_name.as_str());

        match self
            .docker
            .create_container(Some(container_options.clone()), config)
            .await
        {
            Ok(container) => {
                self.docker
                    .start_container(&container.id, None::<StartContainerOptions<String>>)
                    .await
                    .unwrap();

                let container_properties: ContainerProperties = ContainerProperties {
                    id: container.id.clone(),
                    name: container_options.name.clone().to_owned(),
                    is_running: true,
                    created: true,
                    started: true,
                    ssh_key: key_path,
                };
                Ok(container_properties)
            }
            Err(error) => {
                self.clean_up(&container_name).await;
                Err(error)
            }
        }
    }

    // TODO: On this configuration, need to add exposed ports
    // healthcheck (how can we tell whether the container is healthy or not?) [figure this out]
    // Add network configuration as well
    pub fn create_config(&self) -> Config<&str> {
        Config {
            image: Some("kowry_ubuntu"),
            exposed_ports: Some(HashMap::from([("22/tcp", HashMap::new())])),
            host_config: Some(HostConfig {
                port_bindings: Some(HashMap::from([(
                    "22/tcp".to_string(),
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some("2222".to_string()),
                    }]),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    // TODO: Rethink the criteria for naming the container
    // make the argument optional and create a naming convention
    // Maybe based on the user as well (because that might be how we manage the containers)
    pub fn create_container_options(&self, container_name: &str) -> CreateContainerOptions<String> {
        CreateContainerOptions {
            name: String::from(container_name),
            platform: Some(format!("arm64")),
        }
    }

    async fn image_exists(&self, image: &str) -> bool {
        match self.docker.inspect_image(image).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn create_ssh_key(&self, container_name: &str) -> Result<String, String> {
        let save_name = format!(".keys/{}", container_name);
        // match self.key_generator.generate_key(&save_name).await {
        //     true => Ok(save_name),
        //     false => Err("Failed to generate SSH key".to_string()),
        // }
        Ok("".to_string())
    }

    async fn clean_up(&self, container_name: &str) {
        let save_name = format!(".keys/{}", container_name);
        //self.key_generator.delete_key(&save_name).await;
    }
}
