use std::{
    env,
    io::{stdin, stdout, Write},
    process::{Command, Stdio},
};

use tempfile::Builder;

enum ArgState {
    Package,
    None,
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let working_dir = env::current_dir()?;
    let path_str = args.get(1).ok_or(Error::NoArguments)?;
    let path = working_dir.join(path_str);
    let bind = std::fs::read_to_string(path)?;
    let source = bind.split("\n");
    let mut packages = vec![];

    let mut kwargs = args[2..].iter();
    while let Some(arg) = kwargs.next() {
        let state = match arg.as_str() {
            "-p" => ArgState::Package,
            _ => ArgState::None,
        };

        match state {
            ArgState::Package => {
                let package = kwargs.next().ok_or(Error::ExpectedPackage)?;
                packages.push(package);
            }
            _ => {}
        }
    }

    println!(
        "{:?}",
        Command::new("cabal")
            .arg("install")
            .arg("--lib")
            .arg("--package-env")
            .arg(".")
            .args(packages)
            .output()?
    );

    let mut target = Builder::new().suffix(".hs").tempfile()?;

    println!("{:?}", args);

    let mut save = false;
    for line in source {
        if line.to_lowercase().starts_with("```haskell") {
            save = true;
            continue;
        } else if save && line.starts_with("```") {
            save = false;
            continue;
        }
        if save {
            target.write_all(line.as_bytes())?;
            target.write_all("\n".as_bytes())?;
        }
    }

    let mut command = Command::new("ghci")
        .stdin(Stdio::piped())
        .arg(target.path())
        .spawn()?;
    let pipe = command.stdin.as_mut().ok_or(Error::StdinError)?;

    loop {
        let mut s = "".to_string();
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        pipe.write_all(s.as_bytes())?;
        if s.replace("\r", "").replace("\n", "") == ":q" {
            break;
        }
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to open stdin")]
    StdinError,
    #[error("No arguments provided")]
    NoArguments,
    #[error("Expected package name after \"-p\"")]
    ExpectedPackage,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
