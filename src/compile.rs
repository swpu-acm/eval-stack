use std::{
    env, ffi::OsStr, path::{Path, PathBuf}, process::Stdio
};

use anyhow::Result;
use tokio::{fs::File, io, process::Command};

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Language {
    #[default]
    Rust,
    C,
    CPP,
    Python,
    NodeJs,
    Golang,
    Java,
}

pub async fn compile<B: Into<PathBuf>, S: Into<PathBuf>, O: AsRef<str>>(
    language: Language,
    base: B,
    source_file_path: S,
    output_file: O,
) -> Result<()> {
    let base_path = Into::<PathBuf>::into(base);
    let source_path = Into::<PathBuf>::into(source_file_path);
    let source_path_str = source_path.to_string_lossy();
    let output_path = base_path.join(output_file.as_ref());
    let output_path_str = output_path.to_string_lossy();

    let command = match language {
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
            Some(command)
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
            Some(command)
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
            Some(command)
        }
        Language::Python => {
            let mut command = Command::new("python3");
            command.args(["-m", "py_compile", source_path_str.as_ref()]);
            Some(command)
        }
        Language::NodeJs => None,
        Language::Golang => {
            let mut command = Command::new("go");
            command.args([
                "build",
                "-o",
                output_path_str.as_ref(),
                source_path_str.as_ref(),
            ]);
            Some(command)
        }
        Language::Java => {
            let mut command = Command::new("javac");
            let java_path = base_path.join("Main.java");
            if source_path.file_name() != Some(OsStr::new("Main.java")) {
                io::copy(
                    &mut File::open(source_path_str.as_ref()).await?,
                    &mut File::create(&java_path).await?,
                )
                .await?;
            }
            command.arg(java_path.to_string_lossy().as_ref());
            Some(command)
        }
    };

    if let Some(mut command) = command {
        command.kill_on_drop(true).stdout(Stdio::piped()).spawn()?;

        let output = command.output().await?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(anyhow::anyhow!(
                "Failed to compile source code: {}",
                error_message
            ));
        }
    }
    Ok(())
}
