#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
cargo_metadata = "0.22.0"
clap = { version = "4.5", features = ["derive"] }
color-eyre = "0.6.5"
duct = "1.1.0"
---

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, OptionExt};
use duct::cmd;

#[derive(Debug, Parser)]
struct CliArgs {
    #[command(subcommand)]
    task: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
    Build,
    Install { install_dir: PathBuf },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    match CliArgs::parse().task {
        Task::Build => {
            task_step("Building with debug profile");
            let _ = cmd!("cargo", "build").run()?;

            task_step("Build complete");
        },
        Task::Install { install_dir } => {
            task_step("Getting workspace metadata");
            let metadata = get_cargo_metadata()?;
            let Some(binary_name) = get_binary_name(&metadata) else {
                eyre::bail!("No binary configured");
            };

            task_step("Building with release profile");
            let _ = cmd!("cargo", "build", "--release").run()?;

            task_step("Copying to install directory");
            let src_path = metadata
                .workspace_root
                .as_std_path()
                .join("target")
                .join("release")
                .join(&binary_name);
            let dst_path = install_dir.canonicalize()?.join(&binary_name);
            std::fs::copy(src_path, &dst_path)?;
            task_step(format!("Installed to `{}`", dst_path.display()));
        },
    }

    Ok(())
}

// == HELPER FUNCTIONS

fn task_step<M: std::fmt::Display>(msg: M) {
    println!("==> {msg}");
}

const CARGO_METADATA: std::sync::LazyLock<Option<cargo_metadata::Metadata>> =
    std::sync::LazyLock::new(|| cargo_metadata::MetadataCommand::new().exec().ok());

fn get_cargo_metadata() -> color_eyre::Result<cargo_metadata::Metadata> {
    std::sync::LazyLock::force(&CARGO_METADATA)
        .clone()
        .ok_or_eyre("Could not obtain metadata for the current cargo workspace")
}

fn get_binary_name(metadata: &cargo_metadata::Metadata) -> Option<String> {
    metadata.root_package()?.targets.iter().find_map(|target| {
        if target.is_bin() {
            if cfg!(windows) {
                Some(format!("{tname}.exe", tname = target.name))
            } else {
                Some(target.name.to_owned())
            }
        } else {
            None
        }
    })
}
