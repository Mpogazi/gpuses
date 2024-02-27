use super::images::Images;
use super::system::SystemInfo;

pub struct Invoker {
    system_info: SystemInfo,
    images: Images
}

impl Invoker {
    pub fn new() -> Invoker {
        Invoker {
            system_info: SystemInfo::new(),
            images: Images::new()
        }
    }

    pub async fn start_image(&mut self) {
        self.system_info.show();
        self.images.start_image().await.unwrap();
    }
}