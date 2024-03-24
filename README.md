# Gnome Shell Extension Installer

The Gnome Shell Extension Installer is a command-line tool written in Rust that simplifies the process of installing and enabling the AppIndicator support extension for the Gnome Shell desktop environment.

## Features

- Checks for required dependencies (git, meson, ninja, gnome-extensions)
- Installs any missing dependencies automatically
- Clones the gnome-shell-extension-appindicator repository
- Builds the extension using meson and ninja
- Enables the extension using gnome-extensions

## Installation

### Prerequisites

- Rust (version 1.59.0 or later)
- Cargo (Rust's package manager)

### Steps

1. Clone the repository:

   ```
   git clone https://github.com/mranv/gnome-shell-extension-installer.git
   ```

2. Navigate to the project directory:

   ```
   cd gnome-shell-extension-installer
   ```

3. Build the project:

   ```
   cargo build --release
   ```

4. Run the binary:

   ```
   sudo ./target/release/gnome_shell_extension_installer
   ```

   Note: The application requires root privileges to install dependencies and enable the extension.

## Usage

After running the binary, the application will guide you through the installation process. It will check for dependencies, install any missing ones, clone the repository, build the extension, and finally enable it.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the project's GitHub repository.
