use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bundle-runner <path-to-application.bundle>");
        std::process::exit(1);
    }

    let bundle_path_str = &args[1];
    let bundle_path = Path::new(bundle_path_str);

    if !bundle_path.is_dir() {
        eprintln!("Error: '{}' is not a valid bundle directory.", bundle_path_str);
        std::process::exit(1);
    }

    if bundle_path.extension().and_then(|s| s.to_str()) != Some("bundle") {
        eprintln!("Error: Target must have a '.bundle' extension.");
        std::process::exit(1);
    }

    let relative_exec_path = bundle_path.join("AppRun");

    let executable_path = match std::fs::canonicalize(&relative_exec_path) {
        Ok(path) => path,
        Err(_) => {
            eprintln!(
                "Error: Invalid bundle format. Missing internal executable at '{:?}'", 
                relative_exec_path
            );
            std::process::exit(1);
        }
    };

    println!("Launching bundle: {}...", bundle_path_str);

    let app_args: Vec<&String> = args.iter().skip(2).collect();

    let mut child = Command::new(&executable_path)
        .args(&app_args)
        .current_dir(bundle_path) 
        .spawn()
        .expect("Failed to start the bundle application");

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
