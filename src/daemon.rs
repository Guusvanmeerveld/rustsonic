use std::fs::{create_dir_all, File};

use daemonize::Daemonize;
use dirs;

use crate::constants;

pub fn start_daemon() {
    let user = whoami::username();

    let working_dir = format!(
        "{}/{}/run",
        dirs::data_local_dir().unwrap().display(),
        constants::APPLICATION_NAME
    );

    create_dir_all(&working_dir).unwrap();

    let stdout = File::create(format!("{}/daemon.out", working_dir)).unwrap();
    let stderr = File::create(format!("{}/daemon.err", working_dir)).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(format!("{}/process.pid", working_dir))
        .working_directory(working_dir)
        .user(&*user)
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => println!("Success, starting daemon"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
