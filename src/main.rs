extern crate mysqlclient_sys as ffi;
use dotenv::dotenv;
use library_checker::{check_libraries, check_library};
use std::ffi::{CStr, CString};
use std::{env, ptr};

mod color;
mod library_checker;
mod version_utils;

fn main() {
    // Load environment varibles from .env file
    dotenv().ok();

    // Get environment varibles
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_user = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_port: u32 = env::var("DB_PORT")
        .expect("DB_PORT must be set")
        .parse()
        .expect("DB_PORT must be a number");
    let mysql_version = env::var("MYSQLCLIENT_VERSION").expect("MYSQLCLIENT_VERSION must be set");

    version_utils::print_mysql_client_env();
    version_utils::print_rust_versions();

    // dependiendo de la versión de mysql
    // libcrypto-3 tiene diferentes numeros al igual que
    // check_library("libcrypto-3-x64.dll");
    // hacer el checkeo solo si mysql_version es mayor a la 8.0.17
    // Check the MySQL version
    if compare_versions(&mysql_version, "8.0.17") {
        // If the MySQL version is greater than 8.0.17, check the libraries
        let libraries = ["libcrypto-3-x64.dll", "libssl-3-x64.dll"];
        check_libraries(&libraries);
    } else {
        println!("MySQL version is less than or equal to 8.0.17, skipping library checks.");
    }

    unsafe {
        // Init MySQL
        let mysql = ffi::mysql_init(ptr::null_mut());
        if mysql.is_null() {
            eprintln!(
                "{}Error initializing MySQL{}",
                color::error_setup_color(),
                color::normal_color()
            );
            return;
        }

        // Connect to the db
        let host = CString::new(db_host).unwrap();
        let user = CString::new(db_user).unwrap();
        let pass = CString::new(db_pass).unwrap();
        let dbname = CString::new(db_name).unwrap();
        let port = db_port;

        if ffi::mysql_real_connect(
            mysql,
            host.as_ptr(),
            user.as_ptr(),
            pass.as_ptr(),
            dbname.as_ptr(),
            port,
            ptr::null(),
            0,
        )
        .is_null()
        {
            eprintln!(
                "{}Error connecting to the database: {}{}",
                color::error_color(),
                CStr::from_ptr(ffi::mysql_error(mysql)).to_str().unwrap(),
                color::normal_color()
            );
            ffi::mysql_close(mysql);
            return;
        }

        println!(
            "\n{}Successfully connected to the database!{}",
            color::success_color(),
            color::normal_color()
        );

        // Close connection
        ffi::mysql_close(mysql);
    }
}

fn compare_versions(version1: &str, version2: &str) -> bool {
    let v1_parts: Vec<u32> = version1.split('.').filter_map(|s| s.parse().ok()).collect();
    let v2_parts: Vec<u32> = version2.split('.').filter_map(|s| s.parse().ok()).collect();

    for (v1, v2) in v1_parts.iter().zip(v2_parts.iter()) {
        if v1 > v2 {
            return true;
        } else if v1 < v2 {
            return false;
        }
    }

    v1_parts.len() > v2_parts.len()
}
