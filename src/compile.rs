use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
};

use anyhow::Result;
use tokio::process::Command;

#[derive(Clone, Copy)]
pub enum Language {
    C,
    CPP,
    Rust,
    Python,
    Go,
}

pub async fn compile<B: Into<PathBuf>, E: AsRef<str>, O: AsRef<str>>(
    language: Language,
    base: B,
    source_file: E,
    output_file: O,
) -> Result<()> {
    let base_path = Into::<PathBuf>::into(base);
    let source_path = base_path.join(source_file.as_ref());
    let source_path_str = source_path.to_string_lossy();
    let output_path = base_path.join(output_file.as_ref());
    let output_path_str = output_path.to_string_lossy();

    let mut command = match language {
        Language::C => {
            let mut command = Command::new("gcc");
            command.args([
                "-O2",
                "-w",
                "-fmax-errors=3",
                "-std=c17",
                source_path_str.as_ref(),
                "-lm",
                "-o",
                output_path_str.as_ref(),
            ]);
            command
        }
        Language::CPP => {
            let mut command = Command::new("g++");
            command.args([
                "-O2",
                "-w",
                "-fmax-errors=3",
                "-std=c++20",
                source_path_str.as_ref(),
                "-lm",
                "-o",
                output_path_str.as_ref(),
            ]);
            command
        }
        Language::Rust => {
            let rustc_path = Path::new(&env::var("HOME")?)
                .join(".rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc");
            if !rustc_path.exists() {
                panic!("Failed to find rustc from {}", rustc_path.display());
            }
            let mut command = Command::new(rustc_path);
            command.args([
                "--edition=2021",
                source_path_str.as_ref(),
                "-C",
                "embed-bitcode=no",
                "-C",
                "opt-level=2",
                "-o",
                output_file.as_ref(),
            ]);
            command
        }
        Language::Python => {
            let mut command = Command::new("python3");
            command.args(["-m", "py_compile", source_path_str.as_ref()]);
            command
        }
        Language::Go => todo!(),
    };

    command.kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

    let output = command.output().await?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        Err(anyhow::anyhow!(
            "Failed to compile source code: {}",
            error_message
        ))
    } else {
        Ok(())
    }
}
