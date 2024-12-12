use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use derive_builder::Builder;

#[derive(Error, Debug)]
pub enum PackerError {
    #[error("Failed to execute Packer command: {0}")]
    ExecutionError(String),
    #[error("Failed to find Packer executable")]
    NotFound,
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, PackerError>;

#[derive(Debug, Clone)]
pub struct Packer {
    executable: PathBuf,
    working_dir: Option<PathBuf>,
}

#[derive(Debug, Builder)]
pub struct BuildOptions {
    #[builder(default)]
    pub parallel_builds: Option<i32>,
    #[builder(default)]
    pub debug: bool,
    #[builder(default)]
    pub force: bool,
    #[builder(default)]
    pub timestamp_ui: bool,
    #[builder(default)]
    pub color: bool,
    #[builder(default)]
    pub vars: Vec<(String, String)>,
    #[builder(default)]
    pub var_files: Vec<PathBuf>,
}

impl Default for BuildOptions {
    fn default() -> Self {
        BuildOptions {
            parallel_builds: None,
            debug: false,
            force: false,
            timestamp_ui: false,
            color: true,
            vars: Vec::new(),
            var_files: Vec::new(),
        }
    }
}

impl Packer {
    /// Create a new Packer instance
    pub fn new() -> Result<Self> {
        let executable = if cfg!(target_os = "windows") {
            PathBuf::from("./packer.exe")
        } else {
            PathBuf::from("./packer")
        };

        if !executable.exists() {
            return Err(PackerError::NotFound);
        }

        Ok(Self {
            executable,
            working_dir: None,
        })
    }

    /// Set working directory for Packer commands
    pub fn with_working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Build images using a template
    pub fn build<P: AsRef<std::path::Path>>(&self, template: P, options: &BuildOptions) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("build");

        if options.debug {
            cmd.arg("-debug");
        }
        if options.force {
            cmd.arg("-force");
        }
        if let Some(parallel) = options.parallel_builds {
            cmd.args(["-parallel-builds", &parallel.to_string()]);
        }
        if !options.color {
            cmd.arg("-color=false");
        }
        if options.timestamp_ui {
            cmd.arg("-timestamp-ui");
        }

        // Add variables
        for (key, value) in &options.vars {
            cmd.arg(format!("-var={}={}", key, value));
        }

        // Add var files
        for var_file in &options.var_files {
            cmd.arg(format!("-var-file={}", var_file.display()));
        }

        cmd.arg(template.as_ref());

        self.execute_command(cmd)
    }

    /// Initialize a new Packer configuration
    pub fn init<P: AsRef<std::path::Path>>(&self, template: P) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("init").arg(template.as_ref());
        self.execute_command(cmd)
    }

    /// Validate a Packer template
    pub fn validate<P: AsRef<std::path::Path>>(&self, template: P) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("validate").arg(template.as_ref());
        self.execute_command(cmd)
    }

    /// Inspect a template
    pub fn inspect<P: AsRef<std::path::Path>>(&self, template: P) -> Result<String> {
        let mut cmd = self.base_command();
        cmd.arg("inspect").arg(template.as_ref());
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(PackerError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Fix template
    pub fn fix<P: AsRef<std::path::Path>>(&self, template: P) -> Result<String> {
        let mut cmd = self.base_command();
        cmd.arg("fix").arg(template.as_ref());
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(PackerError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get version information
    pub fn version(&self) -> Result<String> {
        let mut cmd = self.base_command();
        cmd.arg("version");
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(PackerError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Create a base command with common configuration
    fn base_command(&self) -> Command {
        let mut cmd = Command::new(&self.executable);
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        cmd
    }

    /// Execute a command and handle its result
    fn execute_command(&self, mut cmd: Command) -> Result<()> {
        let status = cmd.status()?;
        
        if !status.success() {
            return Err(PackerError::ExecutionError(
                format!("Command failed with exit code: {}", status)
            ));
        }

        Ok(())
    }
}

// Plugin management functionality
impl Packer {
    /// Install a Packer plugin
    pub fn plugin_install(&self, plugin_name: &str) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.args(["plugin", "install", plugin_name]);
        self.execute_command(cmd)
    }

    /// Remove a Packer plugin
    pub fn plugin_remove(&self, plugin_name: &str) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.args(["plugin", "remove", plugin_name]);
        self.execute_command(cmd)
    }

    /// List installed plugins
    pub fn plugin_list(&self) -> Result<String> {
        let mut cmd = self.base_command();
        cmd.args(["plugin", "list"]);
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(PackerError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

// Console functionality
impl Packer {
    /// Start Packer console
    pub fn console<P: AsRef<std::path::Path>>(&self, template: P) -> Result<()> {
        let mut cmd = self.base_command();
        cmd.arg("console").arg(template.as_ref());
        self.execute_command(cmd)
    }
}

// HCL2 upgrade functionality
impl Packer {
    /// Upgrade HCL2 configuration
    pub fn hcl2_upgrade<P: AsRef<std::path::Path>>(&self, template: P) -> Result<String> {
        let mut cmd = self.base_command();
        cmd.arg("hcl2_upgrade").arg(template.as_ref());
        let output = cmd.output()?;
        
        if !output.status.success() {
            return Err(PackerError::ExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper function to create a test environment
    fn setup_test_env() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_build_options_builder() {
        let options = BuildOptionsBuilder::default()
            .debug(true)
            .force(true)
            .parallel_builds(Some(2))
            .vars(vec![("key".to_string(), "value".to_string())])
            .build()
            .unwrap();

        assert!(options.debug);
        assert!(options.force);
        assert_eq!(options.parallel_builds, Some(2));
        assert_eq!(options.vars.len(), 1);
        assert_eq!(options.vars[0].0, "key");
        assert_eq!(options.vars[0].1, "value");
    }

    #[test]
    fn test_packer_new_not_found() {
        // Create a clean test directory
        let test_dir = setup_test_env();
        
        // Save current dir and change to test dir
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(test_dir.path()).unwrap();
        
        // Now we know for sure there's no packer executable here
        let packer = Packer::new();
        assert!(packer.is_err());
        
        // Change back to original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_packer_with_working_dir() {
        let test_dir = setup_test_env();
        let packer = Packer {
            executable: PathBuf::from("dummy"),
            working_dir: None,
        }.with_working_dir(test_dir.path());
        
        assert_eq!(packer.working_dir.unwrap(), test_dir.path());
    }

    #[test]
    fn test_build_options_default() {
        let options = BuildOptions::default();
        assert!(!options.debug);
        assert!(!options.force);
        assert!(options.color);
        assert!(options.vars.is_empty());
        assert!(options.var_files.is_empty());
        assert_eq!(options.parallel_builds, None);
    }

    #[test]
    fn test_build_command_construction() {
        let packer = Packer {
            executable: PathBuf::from("dummy"),
            working_dir: None,
        };

        let options = BuildOptionsBuilder::default()
            .debug(true)
            .force(true)
            .parallel_builds(Some(2))
            .vars(vec![("region".to_string(), "us-west-2".to_string())])
            .var_files(vec![PathBuf::from("vars.json")])
            .build()
            .unwrap();

        let cmd = packer.base_command();
        // We can't test the full command execution, but we can verify the struct is set up correctly
        assert_eq!(cmd.get_program(), PathBuf::from("dummy"));
    }
}