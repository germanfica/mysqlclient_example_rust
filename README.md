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
- Basic knowledge of Rust
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

## `mysqlclient-sys` Runtime Error (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)

If you encounter the error:

![WindowsTerminal_tJwkxZMmCu](https://github.com/user-attachments/assets/2df34b04-14d5-4971-9552-5a2bb5c799e0)

```bash
error: process didn't exit successfully: `target\debug\mysqlclient_example.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
```

This indicates a missing DLL issue. Use [Dependency Walker](https://www.dependencywalker.com/) to identify the missing DLLs. In this case, the problem was missing `LIBCRYPTO-3-X64.DLL` and `LIBSSL-3-X64.DLL` from the MySQL bin directory.

### Solution

Add the MySQL bin directory to your PATH environment variable. For example, with MySQL version 8.0.36:

```bash
PATH=C:\mysql-8.0.36-winx64\bin
MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.36-winx64\lib
MYSQLCLIENT_VERSION=8.0.36
```

Ensure these DLLs are present in the bin directory, or copy them into your project's target folder.

### Explanation of Dependency Walker

Dependency Walker is a tool that inspects and identifies all the DLLs that an executable depends on. It helps by:

1. **Identifying Missing DLLs:** If your application fails to run due to missing dependencies, Dependency Walker shows you exactly which DLLs are missing.
2. **Tracing Load Issues:** You can see how the system tries to load each DLL, which helps pinpoint issues related to PATH configuration or version conflicts.
3. **Fixing Environment Setup:** Once you've identified missing DLLs (such as `LIBCRYPTO-3-X64.DLL` and `LIBSSL-3-X64.DLL`), you can update the PATH environment variable to include the correct directories or manually add the DLLs to the necessary locations.

This tool was essential in resolving the missing dependency issue I faced in this project, allowing to ensure the proper setup of the environment variables and dependencies.

## Tested MySQL versions

- MySQL 8.0.18 (works).
  - MD5: `3c1fc0bc3368639d968fbe5bf8afa23d`
  ```bash
  Path=C:\mysql-8.0.18-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.18-winx64\lib
  MYSQLCLIENT_VERSION=8.0.18
  ```

- MySQL 8.0.19 (works).
  - MD5: `f52c52e7b499958acc5f08ce0a869cab`
  ```bash
  Path=C:\mysql-8.0.19-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.19-winx64\lib
  MYSQLCLIENT_VERSION=8.0.19
  ```

- MySQL 8.0.20 (works).
  - MD5: `1335fe593b055686823fd69c7ef035f5`
  ```bash
  Path=C:\mysql-8.0.20-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.20-winx64\lib
  MYSQLCLIENT_VERSION=8.0.20
  ```

- MySQL 8.0.30 (works).
  - MD5: `d17b3d4bab676a2c365b82f65c9a5374`
  ```bash
  Path=C:\mysql-8.0.30-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.30-winx64\lib
  MYSQLCLIENT_VERSION=8.0.30
  ```

- MySQL 8.0.36 (works).
  - MD5: `a2f95f3625440e61d9012c5e2364bd79`
  ```bash
  Path=C:\mysql-8.0.36-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.36-winx64\lib
  MYSQLCLIENT_VERSION=8.0.36
  ```

- MySQL 8.0.37 (works).
  - MD5: `936fc116f2dd865dc26ef3d71f5730e8`
  ```bash
  Path=C:\mysql-8.0.37-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.37-winx64\lib
  MYSQLCLIENT_VERSION=8.0.37
  ```

- MySQL 8.0.39 (works).
  - MD5: `bfba039694efb85916830cf9dc65ccc8`
  ```bash
  Path=C:\mysql-8.0.39-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.0.39-winx64\lib
  MYSQLCLIENT_VERSION=8.0.39
  ```

- MySQL 8.4.0 (works).
  - MD5: `23fa293db80b5d49f9e97d34c2037a82`
  ```bash
  Path=C:\mysql-8.4.0-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.4.0-winx64\lib
  MYSQLCLIENT_VERSION=8.4.0
  ```

- MySQL 8.4.2 (works).
  - MD5: `9aad84967d8a94c390e76366ca85ec3c`
  ```bash
  Path=C:\mysql-8.4.2-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.4.2-winx64\lib
  MYSQLCLIENT_VERSION=8.4.2
  ```

- MySQL 9.0.1 (works).
  - MD5: `bfba039694efb85916830cf9dc65ccc8`
  ```bash
  Path=C:\mysql-8.4.2-winx64\bin
  MYSQLCLIENT_LIB_DIR=C:\mysql-8.4.2-winx64\lib
  MYSQLCLIENT_VERSION=8.4.2
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
- https://github.com/sgrif/mysqlclient-sys/issues/57
