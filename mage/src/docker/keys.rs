use std::process::{Command, Stdio};
pub struct KeyGenerator;

impl KeyGenerator {
    pub fn new() -> KeyGenerator {
        KeyGenerator
    }

    // TODO: Figure out whether there is a rust library wrapper
    // for the ssh-keygen command. Otherwise, we might have to
    // implement terrible approaches to inputing the passphrase
    // for the key generation process.
    pub async fn generate_key(&self, save_name: &str) -> bool {
        let mut command = Command::new("ssh-keygen");
        command.args(["-f", save_name]);
        command.stdin(Stdio::piped());

        match command.status() {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }

    // TODO: Implement the function to reverse the key generation process (delete the keys)
    // If any of the steps in creating/building an image failed.
    // Because orphaned keys are a security risk. Because we might want to use the key file name
    // to figure out which keys belong to who.

    pub async fn delete_key(&self, save_name: &str) -> bool {
        let mut command = Command::new("rm");
        command.args(["-f", save_name, &format!("{}.pub", save_name)]);

        match command.status() {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }
}
