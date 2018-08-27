use std::process::Command;

fn main() {
    // Instead of a dependency on the project, invoke its build manually.
    // Needed because they are different targets.
    Command::new("./build")
        .current_dir("m4_firmware/")
        .output()
        .expect("Failed to run M4 build script");

    println!("cargo:rerun-if-changed=m4_firmware");
    println!("cargo:rerun-if-changed=m4_firmware");

    println!("cargo:rustc-link-search=native=m4_firmware/target/thumbv7em-none-eabi/release");
    println!("cargo:rustc-link-lib=static=m4archive");
}
