use bollard::Docker;
use bollard::image::ListImagesOptions;
use std::default::Default;

pub struct Invoker;

impl Invoker {
    pub fn new() -> Invoker {
        Invoker
    }

    pub async fn images(&self) {
        let docker = Docker::connect_with_local_defaults().unwrap();
        let images = &docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await.unwrap();
        
        for image in images {
            println!("-> {:?}", image);
        }
    }
}