use std::fs;

fn create_backup_output_dir(path: &String) {
    fs::DirBuilder::new()
        .create(path).unwrap();
}

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_directory() {
        let path = "/tmp/output";
        create_backup_output_dir(&String::from(path));
        assert!(fs::metadata(path).unwrap().is_dir());

        fs::remove_dir(path).unwrap();
    }
}
