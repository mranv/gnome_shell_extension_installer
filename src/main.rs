use std::process::{Command, Stdio};
use std::io::{self, Write};

fn check_dependency(command: &str, args: &[&str]) -> bool {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn main() {
    let dependencies = [
    ("git", &["--version"]),
    ("meson", &["--version"]),
    ("ninja", &["--version"]),
    ("gnome-extensions", &["--version"]),
];

let mut missing_dependencies = Vec::new();

for (command, args) in &dependencies {
    if !check_dependency(command, *args) {
        missing_dependencies.push(command);
    }
}
    if !missing_dependencies.is_empty() {
        println!("Missing dependencies: {:?}", missing_dependencies);
        println!("Installing missing dependencies...");

        // Install missing dependencies (assuming apt package manager for simplicity)
        let install_cmd = Command::new("sudo")
            .arg("apt")
            .arg("install")
            .args(&missing_dependencies)
            .output()
            .expect("Failed to execute apt install command.");

        if !install_cmd.status.success() {
            println!("Failed to install dependencies.");
            io::stdout().write_all(&install_cmd.stdout).unwrap();
            io::stderr().write_all(&install_cmd.stderr).unwrap();
            return;
        }

        println!("Dependencies installed successfully.");
    }

    // Clone the repository
    let git_clone_output = Command::new("git")
        .args(&["clone", "https://github.com/ubuntu/gnome-shell-extension-appindicator.git"])
        .output()
        .expect("Failed to execute git clone command.");

    if !git_clone_output.status.success() {
        println!("Failed to clone the repository:");
        io::stdout().write_all(&git_clone_output.stdout).unwrap();
        io::stderr().write_all(&git_clone_output.stderr).unwrap();
        return;
    }

    // Build the extension
    let meson_output = Command::new("meson")
        .args(&["gnome-shell-extension-appindicator", "/tmp/g-s-appindicators-build"])
        .output()
        .expect("Failed to execute meson command.");

    if !meson_output.status.success() {
        println!("Failed to run meson:");
        io::stdout().write_all(&meson_output.stdout).unwrap();
        io::stderr().write_all(&meson_output.stderr).unwrap();
        return;
    }

    let ninja_output = Command::new("ninja")
        .args(&["-C", "/tmp/g-s-appindicators-build", "install"])
        .output()
        .expect("Failed to execute ninja command.");

    if !ninja_output.status.success() {
        println!("Failed to run ninja:");
        io::stdout().write_all(&ninja_output.stdout).unwrap();
        io::stderr().write_all(&ninja_output.stderr).unwrap();
        return;
    }

    // Enable the extension
    let enable_output = Command::new("gnome-extensions")
        .args(&["enable", "appindicatorsupport@rgcjonas.gmail.com"])
        .output()
        .expect("Failed to execute gnome-extensions enable command.");

    if !enable_output.status.success() {
        println!("Failed to enable the extension:");
        io::stdout().write_all(&enable_output.stdout).unwrap();
        io::stderr().write_all(&enable_output.stderr).unwrap();
        return;
    }

    println!("Extension installed and enabled successfully!");
}
