use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let kernel_name = env::var("CARGO_PKG_NAME").expect("Failed to get pkg_name");

    println!("cargo:rustc-link-arg-bin={kernel_name}=--script=.cargo/linker.ld"); // Use the linker script.
    println!("cargo:rustc-link-arg-bin={kernel_name}=--gc-sections"); // Remove unused sections.

    println!("cargo:rerun-if-changed=.cargo/linker.ld");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
