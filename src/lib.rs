//! This is the `rdd` library, which provides a simple way to interact with the `dd` tool from Rust.
//! The `dd` utility is commonly used for low-level copying and conversion of raw data.
//! This library abstracts the `dd` command's functionality, allowing you to configure and execute `dd`
//! commands programmatically in Rust.
//!
//! # Features:
//! - Allows configuration of input/output files, block size, count, etc.
//! - Provides easy error handling and output capture for the `dd` command.
//! - Helps in performing low-level data copying tasks with flexibility in a Rust-based environment.
//!
//! # Example Usage:
//! ```rust
//! use rdd::Dd;
//!
//! // Define the paths for input and output files
//! let input = "./source_file.txt";  // Path to the source file
//! let output = "./destination_file.txt"; // Path for the destination file
//!
//! // Create a Dd instance, set input, output, and block size, then run the command
//! let mut result = Dd::new("dd")
//!     .input(input)
//!     .output(output)
//!     .bs("4M")
//!     .spawn();  // Execute the dd command
//!
//! // Handle the result of the dd command
//! match result {
//!     Ok(output) => println!("Command succeeded: {}", output),
//!     Err(e) => eprintln!("Error: {}", e),  // Print the error message if the command fails
//! }
//! ```
//!
//! # Modules
//! - `dd`: This module contains the core functionality for interacting with the `dd` tool.

mod dd; // The `dd` module contains the primary logic for interacting with the dd tool.
pub use dd::*; // Re-export the functionality of the `dd` module to make it accessible in the public API of the library.
