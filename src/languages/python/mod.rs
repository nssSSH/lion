use crate::errors::{LionError, command_error};
use crate::utils::*;
use std::process::Command;
use std::env;

pub fn new(file_name: &String) -> Result<(), LionError> {
    writer(file_name, "print(\"Hello World!\")")
}

pub fn get_os() -> &'static str{
    env::consts::OS
}

pub fn is_unix() -> bool {
    matches!(get_os(), "linux" | "macos")
}

pub fn run_cmd(cmd: &str, args: &[String]) -> Result<(), LionError> {
    Command::new(cmd)
        .args(args)
        .status()
        .map_err(|err| command_error(cmd, args.to_vec(), None, err))
        .map(|_| ())

}

pub fn run(file_name: &str) -> Result<(), LionError> {
    let args = vec![file_name.to_owned()];
    let os = get_os();

    if is_unix() {
        run_cmd("python3", &args)?;

        println!("\nRan the code successfully");
        Ok(())
    } else if os == "windows" {
        run_cmd("python", &args)?;

        println!("\nRan the code successfully");
        Ok(())
    } else  {
        Ok(())
    }
}

pub fn dependency(dep: &String) -> Result<(), LionError> {
    if dep.contains(".git") {
        let new_git_url = String::from("git+") + dep.as_str();
        let args = vec!["install".to_string(), new_git_url];

        Command::new("pip")
            .args(&args)
            .status()
            .map_err(|err| command_error("pip", args, None, err))?;
    } else {
        let args = vec!["install".to_string(), dep.clone()];

        Command::new("pip")
            .args(&args)
            .status()
            .map_err(|err| command_error("pip", args, None, err))?;
    }

    Ok(())
}

pub fn proj(proj_name: &String) -> Result<(), LionError> {
    let args = vec![
        "-m".to_string(),
        "venv".to_string(),
        format!("{}/venv", proj_name),
    ];

    if let Err(err) = common_dir(proj_name) {
        eprintln!("Failed to create common directories: {}", err);
    }
    let os = get_os();

    if is_unix(){
        run_cmd("python3", &args)?;

        if let Err(err) = new(&format!("{}/src/main.py", proj_name)) {
            eprintln!("Failed to create Python file: {}", err);
        }

        Ok(())
    } else if os == "windows" {
        run_cmd("python", &args)?;

        if let Err(err) = new(&format!("{}/src/main.py", proj_name)) {
            eprintln!("Failed to create Python file: {}", err);
        }

        Ok(())
    } else {
        Ok(())
    }
}
