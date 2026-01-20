use std::{
    env::var_os,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use clap::CommandFactory;
use clap_complete::{aot::*, generate_to};

include!("src/config/mod.rs");

fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=src/config/mod.rs");
    let out_dir = PathBuf::from(var_os("OUT_DIR").ok_or(ErrorKind::NotFound)?);

    // HACK: seek to the ./target/release or ./target/debug directory
    // by skipping unnecessary inner directories
    //
    // blocked by https://github.com/rust-lang/cargo/issues/13663
    let out_dir = out_dir
        .parent()
        .and_then(|dir| dir.parent())
        .and_then(|dir| dir.parent())
        .unwrap();

    build_manpages(out_dir)?;
    gen_completions(out_dir)?;

    // HACK: tell src/config/mod.rs that the build process now started, i.e.
    // that it should now also import and use the other dependencies
    // that are not used in build.rs (e.g. gpui)
    println!("cargo:rustc-cfg=feature=\"build-time\"");
    Ok(())
}

fn build_manpages(out_dir: &Path) -> std::io::Result<()> {
    let out_file = out_dir.join(concat!(env!("CARGO_PKG_NAME"), ".1"));

    let man = clap_mangen::Man::new(ArgsWithConfig::command());
    let mut buffer: Vec<u8> = vec![];
    man.render(&mut buffer)?;

    std::fs::write(&out_file, buffer)?;

    Ok(())
}

fn gen_completions(out_dir: &Path) -> std::io::Result<()> {
    let mut cmd = ArgsWithConfig::command();

    // Generate and write completions for bash, zsh and fish
    generate_to(Bash, &mut cmd, env!("CARGO_PKG_NAME"), out_dir)?;
    generate_to(Zsh, &mut cmd, env!("CARGO_PKG_NAME"), out_dir)?;
    generate_to(Fish, &mut cmd, env!("CARGO_PKG_NAME"), out_dir)?;

    Ok(())
}
