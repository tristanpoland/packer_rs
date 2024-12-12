use std::process::Command;
use std::env;

fn main() {
    if !is_packer_installed() {
        install_packer();
    }
}

fn is_packer_installed() -> bool {
    let packer_executable = if cfg!(target_os = "windows") {
        "./packer.exe"
    } else {
        "./packer"
    };

    Command::new(packer_executable)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_packer() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "windows" => {
            Command::new("powershell")
                .arg("-Command")
                .arg("Invoke-WebRequest -Uri https://releases.hashicorp.com/packer/1.7.8/packer_1.7.8_windows_amd64.zip -OutFile packer.zip; Expand-Archive -Path packer.zip -DestinationPath .;")
                .status()
                .expect("Failed to install Packer on Windows");
        }
        "macos" => {
            Command::new("sh")
                .arg("-c")
                .arg("curl -o packer.zip https://releases.hashicorp.com/packer/1.7.8/packer_1.7.8_darwin_amd64.zip && unzip packer.zip")
                .status()
                .expect("Failed to install Packer on macOS");
        }
        "linux" => {
            Command::new("sh")
                .arg("-c")
                .arg("curl -o packer.zip https://releases.hashicorp.com/packer/1.7.8/packer_1.7.8_linux_amd64.zip && unzip packer.zip")
                .status()
                .expect("Failed to install Packer on Linux");
        }
        _ => panic!("Unsupported OS"),
    }
}