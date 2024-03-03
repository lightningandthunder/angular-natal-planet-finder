use std::{env, fs, path::Path};

fn main() {
    let lib_src_path = Path::new("src")
        .join("swe")
        .join("dll")
        .display()
        .to_string();

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    eprintln!("Target architecture: {}", target_arch);

    match target_arch.as_str() {
        "x86_64" => {
            let win_path = Path::new("target").join("x86_64-pc-windows-gnu");
            let full_path = format!("{}", win_path.display().to_string().as_str());
            println!("cargo:rustc-link-search=native={}", full_path);
            println!("cargo:rustc-link-lib=swedll64");
        }
        _ => {
            panic!("Unsupported architecture: {}", target_arch);
        }
    }
}

fn copy_files_to_dir(src: &str, dest: &str, paths: Vec<&Path>) {
    let src_paths = fs::read_dir(src).unwrap();
    for path in src_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap();

        let dest_path = Path::new(dest).join(file_name);
        match fs::copy(path, dest_path) {
            Err(e) => {
                println!("Error copying file: {}", e);
            }
            _ => {}
        };
    }
}
