use libloading::{Error, Library};

fn load_library(lib_path: &str) -> Result<Library, Error> {
    unsafe { Library::new(lib_path) }
}

pub fn check_library(lib_path: &str) {
    match load_library(lib_path) {
        Ok(_) => {
            println!("The library '{}' was loaded successfully.", lib_path);
        }
        Err(err) => {
            eprintln!("Error loading the library '{}': {}", lib_path, err);
            eprintln!("Make sure the library is present on the system.");
            panic!(
                "Could not load the crucial library '{}'. Aborting.",
                lib_path
            );
        }
    }
}

pub fn check_libraries(libraries: &[&str]) {
    for lib in libraries {
        check_library(lib);
    }
}
