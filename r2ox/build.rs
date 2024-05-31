use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let kernel_name = env::var("CARGO_PKG_NAME")?;
    // let target = env::var("TARGET")?;
    // let profile = env::var("PROFILE")?;

    // let build = format!("target/{target}/{profile}");
    // let target_path = format!("{}/boot.o", build);
    // let path = format!("src/asm/{}-boot.asm", &target.split("-").next().unwrap());

    // std::process::Command::new("nasm")
    //     .args(["-f", "elf64", &path, "-o", &format!("../{}", target_path)])
    //     .status()?;

    println!("cargo:rustc-link-arg-bin={kernel_name}=--script=.cargo/linker.ld");
    // println!("cargo:rustc-link-arg-bin={kernel_name}={}", &target_path);
    println!("cargo:rustc-link-arg-bin={kernel_name}=--gc-sections");

    println!("cargo:rerun-if-changed=.cargo/linker.ld");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
    println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed={}", &format!("../{}", &target_path));
    Ok(())
}
