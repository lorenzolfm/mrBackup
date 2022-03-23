use std::process::Command;

fn main() {
    let mut list_dir = Command::new("ls");

    list_dir.status().expect("process failed to execute");
}
