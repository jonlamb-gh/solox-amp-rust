use std::process::Command;

fn main() {
    // Instead of a dependency on the project, invoke its build manually.
    // Needed because they are different targets.
    Command::new("./build")
        .current_dir("m4-firmware/")
        .output()
        .expect("Failed to run M4 build script");

    println!("cargo:rerun-if-changed=m4-firmware");
    println!("cargo:rerun-if-changed=m4-firmware");

    println!("cargo:rustc-link-search=native=m4-firmware/target/thumbv7em-none-eabi/release");
    println!("cargo:rustc-link-lib=static=m4archive");
}
