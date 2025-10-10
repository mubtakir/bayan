//! # AlBayan Programming Language Compiler
//!
//! Main entry point for the AlBayan compiler and runtime.

use albayan_lib::cli::CliApp;
use std::process;

#[tokio::main]
async fn main() {
    // Initialize the global runtime
    if let Err(e) = albayan_lib::runtime::init_global_runtime() {
        eprintln!("Failed to initialize runtime: {}", e);
        process::exit(1);
    }

    // Create and run the CLI application
    let app = CliApp::new();

    if let Err(e) = app.run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
