use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct SystemInfo {
    system: System,
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            ),
        }
    }

    pub fn idle_cpu(&mut self) -> f32 {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu();
        (100.00 - self.system.global_cpu_info().cpu_usage()) as f32
    }

    pub fn idle_gpu(&mut self) {}
}
