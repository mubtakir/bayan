//! # System Interface Module
//! 
//! This module provides system-level operations for the AlBayan runtime.

use std::io::{self, Write};
use std::fs;
use std::path::Path;
use super::RuntimeError;

/// System interface for I/O and OS operations
#[derive(Debug)]
pub struct SystemInterface {
    /// Standard output buffer
    stdout_buffer: Vec<u8>,
    
    /// Standard error buffer
    stderr_buffer: Vec<u8>,
    
    /// Enable buffering
    buffered: bool,
}

impl SystemInterface {
    /// Create a new system interface
    pub fn new() -> Self {
        Self {
            stdout_buffer: Vec::new(),
            stderr_buffer: Vec::new(),
            buffered: false,
        }
    }
    
    /// Enable or disable output buffering
    pub fn set_buffered(&mut self, buffered: bool) {
        self.buffered = buffered;
        if !buffered {
            self.flush().ok();
        }
    }
    
    /// Print a string to stdout
    pub fn print(&self, s: &str) -> Result<(), RuntimeError> {
        if self.buffered {
            // In a real implementation, we'd need mutable access
            // For now, just print directly
            print!("{}", s);
        } else {
            print!("{}", s);
            io::stdout().flush().map_err(|e| RuntimeError::SystemError(e.to_string()))?;
        }
        Ok(())
    }
    
    /// Print a line to stdout
    pub fn println(&self, s: &str) -> Result<(), RuntimeError> {
        self.print(&format!("{}\n", s))
    }
    
    /// Print to stderr
    pub fn eprint(&self, s: &str) -> Result<(), RuntimeError> {
        eprint!("{}", s);
        if !self.buffered {
            io::stderr().flush().map_err(|e| RuntimeError::SystemError(e.to_string()))?;
        }
        Ok(())
    }
    
    /// Print a line to stderr
    pub fn eprintln(&self, s: &str) -> Result<(), RuntimeError> {
        self.eprint(&format!("{}\n", s))
    }
    
    /// Flush output buffers
    pub fn flush(&self) -> Result<(), RuntimeError> {
        io::stdout().flush().map_err(|e| RuntimeError::SystemError(e.to_string()))?;
        io::stderr().flush().map_err(|e| RuntimeError::SystemError(e.to_string()))?;
        Ok(())
    }
    
    /// Read a file to string
    pub fn read_file(&self, path: &str) -> Result<String, RuntimeError> {
        fs::read_to_string(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to read file '{}': {}", path, e)))
    }
    
    /// Write a string to file
    pub fn write_file(&self, path: &str, content: &str) -> Result<(), RuntimeError> {
        fs::write(path, content)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to write file '{}': {}", path, e)))
    }
    
    /// Append a string to file
    pub fn append_file(&self, path: &str, content: &str) -> Result<(), RuntimeError> {
        use std::fs::OpenOptions;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to open file '{}': {}", path, e)))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| RuntimeError::SystemError(format!("Failed to write to file '{}': {}", path, e)))?;
        
        Ok(())
    }
    
    /// Check if file exists
    pub fn file_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// Check if path is a directory
    pub fn is_directory(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }
    
    /// Create a directory
    pub fn create_directory(&self, path: &str) -> Result<(), RuntimeError> {
        fs::create_dir_all(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to create directory '{}': {}", path, e)))
    }
    
    /// Remove a file
    pub fn remove_file(&self, path: &str) -> Result<(), RuntimeError> {
        fs::remove_file(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to remove file '{}': {}", path, e)))
    }
    
    /// Remove a directory
    pub fn remove_directory(&self, path: &str) -> Result<(), RuntimeError> {
        fs::remove_dir_all(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to remove directory '{}': {}", path, e)))
    }
    
    /// List directory contents
    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, RuntimeError> {
        let entries = fs::read_dir(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to read directory '{}': {}", path, e)))?;
        
        let mut result = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| RuntimeError::SystemError(e.to_string()))?;
            if let Some(name) = entry.file_name().to_str() {
                result.push(name.to_string());
            }
        }
        
        result.sort();
        Ok(result)
    }
    
    /// Get current working directory
    pub fn current_directory(&self) -> Result<String, RuntimeError> {
        std::env::current_dir()
            .map_err(|e| RuntimeError::SystemError(e.to_string()))?
            .to_str()
            .ok_or_else(|| RuntimeError::SystemError("Invalid UTF-8 in path".to_string()))
            .map(|s| s.to_string())
    }
    
    /// Change current working directory
    pub fn change_directory(&self, path: &str) -> Result<(), RuntimeError> {
        std::env::set_current_dir(path)
            .map_err(|e| RuntimeError::SystemError(format!("Failed to change directory to '{}': {}", path, e)))
    }
    
    /// Get environment variable
    pub fn get_env(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
    
    /// Set environment variable
    pub fn set_env(&self, name: &str, value: &str) {
        std::env::set_var(name, value);
    }
    
    /// Get all environment variables
    pub fn get_all_env(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }
    
    /// Execute a system command
    pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<CommandResult, RuntimeError> {
        use std::process::Command;
        
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| RuntimeError::SystemError(format!("Failed to execute command '{}': {}", command, e)))?;
        
        Ok(CommandResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
    
    /// Get current timestamp (milliseconds since epoch)
    pub fn current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
    
    /// Sleep for specified milliseconds
    pub fn sleep(&self, milliseconds: u64) {
        std::thread::sleep(std::time::Duration::from_millis(milliseconds));
    }
    
    /// Get system information
    pub fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            family: std::env::consts::FAMILY.to_string(),
        }
    }
}

/// Result of executing a system command
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub family: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_system_interface_creation() {
        let interface = SystemInterface::new();
        assert!(!interface.buffered);
    }
    
    #[test]
    fn test_file_operations() {
        let interface = SystemInterface::new();
        let test_file = "test_file.txt";
        let test_content = "Hello, AlBayan!";
        
        // Write file
        interface.write_file(test_file, test_content).unwrap();
        assert!(interface.file_exists(test_file));
        
        // Read file
        let content = interface.read_file(test_file).unwrap();
        assert_eq!(content, test_content);
        
        // Clean up
        interface.remove_file(test_file).unwrap();
        assert!(!interface.file_exists(test_file));
    }
    
    #[test]
    fn test_directory_operations() {
        let interface = SystemInterface::new();
        let test_dir = "test_directory";
        
        // Create directory
        interface.create_directory(test_dir).unwrap();
        assert!(interface.is_directory(test_dir));
        
        // List directory (should be empty)
        let contents = interface.list_directory(test_dir).unwrap();
        assert!(contents.is_empty());
        
        // Clean up
        interface.remove_directory(test_dir).unwrap();
        assert!(!interface.file_exists(test_dir));
    }
    
    #[test]
    fn test_environment_variables() {
        let interface = SystemInterface::new();
        
        // Set and get environment variable
        interface.set_env("ALBAYAN_TEST", "test_value");
        let value = interface.get_env("ALBAYAN_TEST");
        assert_eq!(value, Some("test_value".to_string()));
    }
    
    #[test]
    fn test_system_info() {
        let interface = SystemInterface::new();
        let info = interface.get_system_info();
        
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
        assert!(!info.family.is_empty());
    }
}
