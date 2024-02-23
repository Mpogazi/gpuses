use bollard::Docker;
use bollard::image::ListImagesOptions;
use bollard::models::ImageSummary;
use bollard::errors::Error;

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
}