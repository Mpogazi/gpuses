use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::models::PortBinding;
use bollard::secret::HostConfig;
use bollard::Docker;

use rand::Rng;
use std::collections::HashMap;
use std::process::Command;

use super::keys::KeyGenerator;

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
    key_generator: KeyGenerator,
}

impl Images {
    pub fn new() -> Images {
        Images {
            docker: Docker::connect_with_local_defaults().unwrap(),
            key_generator: KeyGenerator::new(),
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
        match self.key_generator.generate_key(&save_name).await {
            true => Ok(save_name),
            false => Err("Failed to generate SSH key".to_string()),
        }
    }

    async fn build_image(&self, key_path: &str) {
        let path = "src/config/Dockerfile";
        let image = "kowry_ubuntu:latest";

        if self.image_exists(image).await {
            println!("Image already exists!");
        } else {
            let status = Command::new("docker")
                .arg("build")
                .arg("--build-arg")
                .arg(format!("SSH_KEY_PATH={}.pub", key_path))
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
        let container_name = format!("container_{}", rand::thread_rng().gen_range(0..10000));
        let key_path = self.create_ssh_key(&container_name).await.unwrap();

        self.build_image(&key_path).await;

        let config = self.create_config();
        let container_options = self.create_container_options(container_name.as_str());

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
            name: container_options.name.clone().to_owned(),
            is_running: true,
            created: true,
            started: true,
            ssh_key: key_path,
        };
        Ok(container_properties)
    }

    pub fn create_config(&self) -> Config<&str> {
        Config {
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
        }
    }

    pub fn create_container_options(&self, container_name: &str) -> CreateContainerOptions<String> {
        CreateContainerOptions {
            name: String::from(container_name),
            platform: Some(format!("arm64")),
        }
    }
}
