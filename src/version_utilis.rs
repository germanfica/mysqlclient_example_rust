// env_utils.rs

use std::env;
use std::process::Command;

pub fn print_mysql_client_env() {
    let mysql_lib_dir = env::var("MYSQLCLIENT_LIB_DIR").expect("MYSQLCLIENT_LIB_DIR must be set");
    let mysql_version = env::var("MYSQLCLIENT_VERSION").expect("MYSQLCLIENT_VERSION must be set");

    println!("MYSQLCLIENT_LIB_DIR: {}", mysql_lib_dir);
    println!("MYSQLCLIENT_VERSION: {}", mysql_version);
}

pub fn print_rust_versions() {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc --version");
    println!(
        "Rustc version: {}",
        String::from_utf8_lossy(&rustc_version.stdout).trim_end()
    );

    let cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute cargo --version");
    println!(
        "Cargo version: {}",
        String::from_utf8_lossy(&cargo_version.stdout).trim_end()
    );
}
