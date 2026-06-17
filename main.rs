// dirx - A bundled application runner for Linux
// Copyright (C) 2026 Alister1-code
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
mod create;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bundle-runner <path-to-application.bundle>");
        std::process::exit(1);
    }
    
    if args[1] == "create"{
        let _create = create::create(args);
        std::process::exit(0);
    }

    let bundle_path_str = &args[1];
    let forwarded_args = &args[2..];
    let bundle_path = Path::new(bundle_path_str);

    if !bundle_path.is_dir() {
        eprintln!("Error: '{}' is not a valid bundle directory.", bundle_path_str);
        std::process::exit(1);
    }

    if bundle_path.extension().and_then(|s| s.to_str()) != Some("bundle") {
        eprintln!("Error: Target must have a '.bundle' extension.");
        std::process::exit(1);
    }

    let config_path = bundle_path.join("AppInfo.toml");
    if !config_path.exists() {
        eprintln!("Error: Missing 'AppInfo.toml' metadata file in bundle root.");
        std::process::exit(1);
    }

    let exec_relative_path = match parse_config_exec_path(&config_path) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error reading configuration: {}", e);
            std::process::exit(1);
        }
    };

    let relative_exec_path = bundle_path.join(exec_relative_path);
    let executable_path = match fs::canonicalize(&relative_exec_path) {
        Ok(path) => path,
        Err(_) => {
            eprintln!(
                "Error: Configured executable not found at '{:?}'", 
                relative_exec_path
            );
            std::process::exit(1);
        }
    };

    println!("Launching bundle: {}...", bundle_path_str);
    let working_dir;
    let assets_dir = bundle_path.join("Assets");
    if parse_uses_assets(&config_path){
        working_dir = if assets_dir.is_dir() {
            assets_dir
        } else {
            bundle_path.to_path_buf()
        };
    } else {
        println!("bundle does not use Assets, keep working directory with the executable");
        working_dir = bundle_path.to_path_buf();
    };
    
    let mut command = Command::new(&executable_path);
    command.args(forwarded_args);
    command.current_dir(working_dir);
    
    println!("Executing Command: {:?}", command);
    
    let mut child = command.spawn().expect("Failed to start the bundle application");
    
    //let mut child = Command::new(&executable_path)
    //    .args(forwarded_args)
    //    .current_dir(working_dir)
    //    .spawn()
    //    .expect("Failed to start the bundle application");

    match child.wait() {
        Ok(status) => {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
        }
        Err(e) => {
            eprintln!("Error while running the application: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_config_exec_path(config_path: &Path) -> Result<PathBuf, String> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("exec_path") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let path_val = parts[1].trim().trim_matches('"').trim_matches('\'');
                return Ok(PathBuf::from(path_val));
            }
        }
    }

    Err("Could not find a valid 'exec_path' key in AppInfo.toml".to_string())
}

fn parse_uses_assets(config_path: &Path) -> bool {
    let content = fs::read_to_string(config_path).unwrap_or_default();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("uses_assets") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let path_val = parts[1].trim().trim_matches('"').trim_matches('\'');
                return path_val == "true";
            }
        }
    }
    false
}
