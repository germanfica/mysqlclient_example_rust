fn main() {
    // Supongamos que tienes los DLLs en `C:\mylibs`:
    // println!("cargo:rustc-link-search=native=C:\\mylibs");

    // Si el DLL es `mylib.dll`, el nombre que debes pasar es `mylib`:
    //println!("cargo:rustc-link-lib=dylib=mylib");
    // println!("cargo:rustc-link-lib=dylib=LIBCRYPTO-3-X64");

}

// use libloading::{Library, Symbol};
use std::ffi::c_void;

// fn main() {
//     // Ruta al DLL
//     let lib_path = "C:\\ruta\\al\\dll\\LIBCRYPTO-3-X64.dll";

//     // Intenta cargar la librería
//     let lib = Library::new(lib_path).unwrap_or_else(|e| {
//         eprintln!("No se pudo cargar {}: {:?}", lib_path, e);
//         std::process::exit(1);
//     });

//     // Si necesitas acceder a funciones dentro del DLL, usa Symbol
//     // Ejemplo: Supongamos que el DLL tiene una función llamada `crypto_function`
//     unsafe {
//         let crypto_function: Symbol<unsafe extern "C" fn() -> c_void> =
//             lib.get(b"crypto_function").expect("No se pudo encontrar 'crypto_function' en el DLL");

//         // Llama a la función
//         crypto_function();
//     }

//     println!("DLL cargado y función ejecutada correctamente.");
// }
