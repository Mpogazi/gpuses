use sysinfo::{System, Components};

pub struct SystemInfo {
    system: System,
    components: Components
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            system: System::new_all(),
            components: Components::new()
        }
    }

    pub fn show(&mut self) {
        self.system.refresh_all();

        let available_cpus = self.system.cpus().iter().filter(|cpu_instance| cpu_instance.cpu_usage() < 90.0);
        
        println!("CPUS available: {}", available_cpus.count());

        self.components.refresh_list();
        println!("GPUs (if available): {}", self.components.list().len());
    }
}