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
        println!("Available memory cpu percentage: {}", self.system_info.idle_cpu());
        self.system_info.idle_gpu();
    }
}