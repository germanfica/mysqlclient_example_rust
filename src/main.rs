extern crate mysqlclient_sys as ffi;
use dotenv::dotenv;
use std::ffi::{CStr, CString};
use std::{env, ptr};

mod color;
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

    version_utils::print_mysql_client_env();
    version_utils::print_rust_versions();

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
