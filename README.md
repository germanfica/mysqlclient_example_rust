# mysqlclient_example

The purpose of this repository is to provide quick code for connection using MySQL. The mysqlclient-sys library is being used, which is currently utilized by Diesel ORM for Rust.

![WindowsTerminal_ipG01pUBY7](https://github.com/user-attachments/assets/bdeec1f4-1001-4651-b421-8ef9efb38f37)

## Getting Started

Follow the steps below to get started with the mysqlclient_example project:

- [Ensure you have all Requirements](#requirements)
- [Set Up MySQL Server](#setup-mysql-server)
- [Download and Configure the Project](#download)
- [Run Tests](#run-tests)

## Requirements

Make sure you have the following installed before starting:

- [MySQL Server](https://dev.mysql.com/downloads/mysql/) (version 8.0.17 recommended)
- Basic knowledge of Rust and Diesel ORM
- Windows 10 Pro (Tested on version 22H2, OS build: 19045.4780)
- [Rust](https://www.rust-lang.org/) (cargo 1.80.1 or newer)

## Download

You can get access to Red Runner source code by using one of the following ways:
- :sparkles: Download Source Code
- :fire: Clone the repository locally:
```bash
git clone https://github.com/germanfica/mysqlclient_example_rust.git
```

## Setup MySQL Server

To configure MySQL Server on your Windows 10 machine, follow these steps:

1. Download and install MySQL Server version 8.0.17 from the official MySQL website.
2. After installation, add the following environment variables:
   - `MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.17-winx64\lib`
   - `MYSQLCLIENT_VERSION=8.0.17`
3. Ensure that these environment variables are correctly set by running:
   ```bash
   echo %MYSQLCLIENT_LIB_DIR%
   echo %MYSQLCLIENT_VERSION%
   ```

## Run Tests

To run the tests:

1. Ensure MySQL Server is running and accessible.
2. In the project directory, execute:
   ```bash
   cargo test
   ```
3. If everything is set up correctly, the tests should pass successfully.

## Tested MySQL versions

C API (libmysqlclient) is included in MySQL 8.0.

- 8.0.0 (works). MD5: `c6d3e54f4eab46d75a845e15f3023d0a`

- 8.0.11 (works). MD5: `0b4efe256a28cd391bf057d4c61ade09`

    ```
    libmysql.dll
    libmysql.lib
    ```

- 8.0.16 (works). MD5: `1a6646b047425cc1150b8a88751e721b`

- 8.0.17 (works). MD5: `d120bb0513c2ccfaeee74b0e99217bb7`

- 8.0.18 (not working). MD5: `3c1fc0bc3368639d968fbe5bf8afa23d`

- 8.0.19 (not working). MD5: `f52c52e7b499958acc5f08ce0a869cab`

- 8.0.20 (not working). MD5: `1335fe593b055686823fd69c7ef035f5`

- 8.0.30 (not working). MD5: `d17b3d4bab676a2c365b82f65c9a5374`

- 8.0.37 (not working). MD5: `936fc116f2dd865dc26ef3d71f5730e8`

- 8.4.2 LTS (not working) - Windows (x86, 64-bit), MSI Installer	- (mysql-8.4.2-winx64.msi)	MD5: `888dc0f177ce11ed461294ff797824c7`

### Output error

The error is unknown, but it is suspected that there may be a bug in the `mysqlclient-sys` binding. In any case, no tests have been done to prove it.

```bash
error: process didn't exit successfully: `target\debug\my_login_app_api.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

### Testing Environment

- Windows 10 Pro. 22H2. OS build: 19045.4780

- diesel = { version = "2.2.3", features = ["mysql"] }

- cargo 1.80.1 (376290515 2024-07-16)

- diesel-cli. Version: 2.2.3. Supported Backends: postgres mysql sqlite

## Links

- https://diesel.rs/
- https://github.com/diesel-rs/diesel
- https://crates.io/crates/mysqlclient-sys
- https://github.com/sgrif/mysqlclient-sys
