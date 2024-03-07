use std::process::Command;
pub struct KeyGenerator;

impl KeyGenerator {
    pub fn new() -> KeyGenerator {
        KeyGenerator
    }

    pub async fn generate_key(&self, save_name: &str) -> bool {
        let mut command = Command::new("ssh-keygen");
        command.args(["-t", "rsa", "-b", "4096", "-f", save_name]);

        match command.status() {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }

    pub async fn clean_up(&self, save_name: &str) -> bool {
        let mut command = Command::new("rm");
        command.args(["-f", save_name, &format!("{}.pub", save_name)]);

        match command.status() {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }
}
