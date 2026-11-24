use rig::{rig_tool, tool::ToolExecutionError};
use std::{
    io,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncBufReadExt, BufReader};

fn file_is_valid(file: &Path) -> Result<(), std::io::Error> {
    log::info!("check allowed {:?}", file);
    if file.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Only relative paths are allowed",
        ));
    }

    let file = file.canonicalize()?;
    log::info!("{file:?}");

    let root = std::env::current_dir()?;
    if !file.starts_with(&root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Path is outside the approved directory",
        ));
    }
    Ok(())
}

async fn call(args: String) -> Result<String, io::Error> {
    let file = Path::new(&args);
    file_is_valid(file)?;
    let metadata = std::fs::metadata(file)?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Only regular files can be read",
        ));
    }

    const MAX_BYTES: u64 = 500_000;
    if metadata.len() > MAX_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File is too large to read",
        ));
    }

    std::fs::read_to_string(file)
}

#[rig::rig_tool(description = "Read file")]
pub async fn read_file(path: String) -> Result<String, ToolExecutionError> {
    log::info!("read_file: {path}");
    call(path).await.map_err(|error| {
        log::error!("{error}");
        ToolExecutionError::other(format!("{error}"))
    })
}

#[rig::rig_tool(description = "clippy")]
pub async fn cargo_clippy() -> Result<String, ToolExecutionError> {
    log::info!("tool: clippy");
    let res = tokio::process::Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-Dwarnings")
        .output()
        .await
        .map_err(|error| {
            log::error!("{error}");
            ToolExecutionError::other(format!("{error}"))
        })?;
    let out = String::from_utf8_lossy(&res.stderr);
    eprintln!("{out}");
    Ok(out.into())
}

#[rig_tool(
    description = "Write content to a file",
    params(file = "File path", content = "Content to write"),
    required(file, content)
)]
pub async fn write_file(file: PathBuf, content: String) -> Result<(), ToolExecutionError> {
    file_is_valid(&file).map_err(|error| {
        log::error!("{error}");
        ToolExecutionError::other(format!("{error}"))
    })?;

    if content.is_empty() {
        return Err(ToolExecutionError::refused("empty"));
    }
    println!("{content}");
    print!("Write to: {file:?} (y/n): ");
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    let _ = reader.read_line(&mut line).await.map_err(|error| {
        log::error!("{error}");
        ToolExecutionError::other(format!("{error}"))
    })?;

    log::info!("=========== {line}");
    if line.starts_with('y') || line.starts_with('Y') {
        log::info!("Writing to file: {file:?}");
        std::fs::write(file, content).map_err(|error| {
            log::error!("{error}");
            ToolExecutionError::other("Write failed")
        })
    } else {
        Err(ToolExecutionError::refused("user said no"))
    }
}
