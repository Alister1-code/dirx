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

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
struct AppInfo {
    name: String,
    exec_path: String,
}

pub fn create(args: Vec<String>) -> std::io::Result<()> {
    if args.len() < 3 {
        eprintln!("Error: Please provide a bundle name.");
        eprintln!("Usage: dirx create <bundle_name>");
        std::process::exit(1);
    }
    
    let bundle_name = &args[2];
    let root_dir = PathBuf::from(format!("{}.bundle", bundle_name));
    
    let assets_dir = root_dir.join("Assets");
    let linux_dir = root_dir.join("Contents/Linux");
    let toml_path = root_dir.join("AppInfo.toml");

    fs::create_dir_all(&assets_dir)?;
    fs::create_dir_all(&linux_dir)?;
    println!("Created directory structure for {}!", root_dir.display());

    let app_info = AppInfo {
        name: bundle_name.clone(),
        exec_path: format!("Contents/Linux/{}", bundle_name), 
    };

    let toml_string = toml::to_string(&app_info)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(toml_path)?;
    file.write_all(toml_string.as_bytes())?;
    
    println!("AppInfo.toml generated successfully.");

    Ok(())
}
