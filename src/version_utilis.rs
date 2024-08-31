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

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_mysql_client_env() {
        // Set up the environment variables
        std::env::set_var("MYSQLCLIENT_LIB_DIR", "mock_lib_dir");
        std::env::set_var("MYSQLCLIENT_VERSION", "mock_version");

        // Redirect stdout to capture the output
        let output = std::panic::catch_unwind(|| {
            print_mysql_client_env();
        });

        assert!(output.is_ok(), "Function panicked");
    }

    #[test]
    fn test_print_rust_versions() {
        // This function is difficult to test as it relies on external commands.
        // You could mock the Command output in a more sophisticated testing setup.
        // For now, we'll ensure it doesn't panic.

        let output = std::panic::catch_unwind(|| {
            print_rust_versions();
        });

        assert!(output.is_ok(), "Function panicked");
    }
}
