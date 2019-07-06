use std::env;
use std::path::Path;

enum Platform {
    Windows,
    // Mac, // TODO - enable once mac has been tested
    Linux,
}

fn get_platform() -> Platform {
    match std::env::var("TARGET").unwrap().split('-').nth(2).unwrap() {
        "win32" | "windows" => Platform::Windows,
        // "darwin" => Platform::Mac,
        "linux" => Platform::Linux,
        other => panic!("Sorry, platform \"{}\" is not supported by CEF.", other),
    }
}

fn choose_source_dir() -> Option<String> {
    if let Ok(path) = env::var("CEF_ROOT") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

fn main() {
    let source_dir = choose_source_dir().expect("Failed to locate CEF lib path");

    // Inform rust what to link
    match get_platform() {
        Platform::Windows => println!("cargo:rustc-link-lib=libcef"),
        Platform::Linux => println!("cargo:rustc-link-lib=cef"),
    };

    println!("Path: {:?}", source_dir);

    let release_dir = Path::new(&source_dir).join("Release");
    let resources_dir = Path::new(&source_dir).join("Resources");

    if !release_dir.exists() {
        panic!(
            "CEF Release directory ({}) does not exist",
            release_dir.to_str().unwrap_or_else(|| "")
        );
    }
    if !resources_dir.exists() {
        panic!(
            "CEF Resources directory ({}) does not exist",
            resources_dir.to_str().unwrap_or_else(|| "")
        );
    }

    if let Some(release_dir) = release_dir.to_str() {
        println!("cargo:rustc-link-search=native={}", release_dir);
    }

    // Copy the required Resources & Release contents to OUT_DIR so that a cargo run works
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../..");

    let opts = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000, // Default
        copy_inside: true,
        depth: 0,
    };

    let mut release_items = fs_extra::dir::get_dir_content(&release_dir).unwrap();
    let mut resources_items = fs_extra::dir::get_dir_content(&resources_dir).unwrap();

    let mut all_items = Vec::new();
    all_items.append(&mut release_items.directories);
    all_items.append(&mut release_items.files);
    all_items.append(&mut resources_items.directories);
    all_items.append(&mut resources_items.files);

    fs_extra::copy_items(&all_items, &dest_path, &opts).unwrap();
}
