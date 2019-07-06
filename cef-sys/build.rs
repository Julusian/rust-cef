use std::path::Path;
use std::env;

fn choose_source_dir() -> Option<String> {
    // Try a global path
    if let Ok(path) = env::var("CEF_PATH") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Try the local lib folder
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir).join("lib");
    if path.exists() {
        return path.to_str().map(|s| s.to_string());
    }

    None
}

fn main() {
    let source_dir = choose_source_dir();
    println!("Path: {:?}", source_dir);

    // Copy the .so files to the deps folder, to make it build
    // TODO
    //    if let Some(path) = source_dir {
    //        let source_path = Path::new(&path);
    //        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
    //        fs::copy(
    //            source_path.join("libndi.so.3"),
    //            dest_path.join("libndi.so.3"),
    //        )
    //        .expect("copy libndi.so.3");
    //
    //        let sl_res =
    //            std::os::unix::fs::symlink(Path::new("libndi.so.3"), dest_path.join("libndi.so"));
    //        if let Err(e) = sl_res {
    //            if e.kind() != ErrorKind::AlreadyExists {
    //                panic!("Unknown error: {}", e);
    //            }
    //        }
    //    }

    //    println!("cargo:rustc-link-search=native={}", cef_path);
    println!("cargo:rustc-link-lib=cef");
    //    println!("cargo:rustc-link-lib=stdc++");
}
