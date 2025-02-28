use std::process::Command;
use thiserror::Error;

/// Enum for errors that can occur when interacting with the 'dd' command.
#[derive(Error, Debug)]
pub enum DdError {
    /// Error when the subprocess for 'dd' cannot be created.
    #[error("An error occurred while creating subprocess for 'dd': {0}")]
    CantRun(std::io::Error),

    /// Error when the 'dd' binary is missing or corrupted.
    #[error("The 'dd' binary is missing or corrupted.")]
    Missing,

    /// Error when converting stdout bytes to a UTF-8 string fails.
    #[error("Unable to convert stdout bytes to UTF-8 String.")]
    InvalidUTF8,

    /// Error for invalid output returned from the 'dd' command.
    #[error("Invalid output returned from 'dd' command.")]
    InvalidOutput,

    /// Error when the 'dd' binary version is older than the minimum required.
    #[error("The binary version is smaller (older) than the min version.")]
    OldVersion,

    /// Error when no input is provided to the 'dd' command.
    #[error("No input given to 'dd' program.")]
    NoInput,
}

/// Struct representing the configuration for running the 'dd' command.
pub struct Dd {
    binary: String,                  // Path to the 'dd' binary.
    min_version: Option<(u16, u16)>, // Optional minimum version for 'dd'.
    input: Option<String>,           // Optional input file.
    output: Option<String>,          // Optional output file.
    options: Vec<String>,            // Additional arguments for the 'dd' command.
}

impl Dd {
    /// Constructs a new `Dd` instance with the given binary path.
    ///
    /// # Parameters
    /// - `binary`: Path to the 'dd' binary executable.
    ///
    /// # Returns
    /// A new instance of `Dd`.
    pub fn new(binary: &str) -> Self {
        Self {
            binary: binary.to_string(),
            input: None,
            output: None,
            min_version: None,
            options: Vec::new(),
        }
    }

    /// Checks if the 'dd' binary exists and is functional by querying its version.
    ///
    /// This method also ensures the binary is at least the minimum required version,
    /// if specified.
    ///
    /// # Returns
    /// - `Ok(())` if the binary is found and the version is valid.
    /// - `Err(DdError)` if any error occurs (e.g., binary not found, version too old, etc.).
    fn check(&self) -> Result<(), DdError> {
        // Run the 'dd' command with the '--version' argument to check its version.
        let cmd = Command::new(&self.binary).arg("--version").output();

        match cmd {
            // If the command fails to run (for example, if 'dd' is missing), return an error.
            Err(_) => Err(DdError::Missing),

            Ok(output) => {
                // If the command ran but was not successful, handle stderr output.
                if !output.status.success() {
                    let stderr =
                        String::from_utf8(output.stderr).map_err(|_| DdError::InvalidUTF8)?;
                    return Err(DdError::CantRun(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        stderr,
                    )));
                }

                // Convert stdout (which contains the version info) into a String.
                let stdout = String::from_utf8(output.stdout);
                if stdout.is_err() {
                    return Err(DdError::InvalidUTF8);
                }

                // Parse the version from stdout.
                let version_str = stdout.unwrap();
                let version_parts: Vec<&str> = version_str
                    .split_whitespace()
                    .nth(2) // Get the version string from the third element.
                    .ok_or(DdError::InvalidOutput)?
                    .split('.')
                    .collect();

                // If the version format is incorrect, return an error.
                if version_parts.len() != 2 {
                    return Err(DdError::InvalidOutput);
                }

                // Parse major and minor version from the version string.
                let version = (
                    version_parts[0].parse::<u16>().unwrap_or(0),
                    version_parts[1].parse::<u16>().unwrap_or(0),
                );

                // Check if the 'dd' binary meets the minimum version requirement.
                if let Some(min_version) = self.min_version {
                    if version < min_version {
                        return Err(DdError::OldVersion);
                    }
                }

                Ok(()) // The 'dd' binary is valid.
            }
        }
    }

    /// Helper method to add key-value arguments to the 'dd' command.
    ///
    /// # Parameters
    /// - `key`: The argument key (e.g., "bs").
    /// - `value`: The corresponding argument value (e.g., "64K").
    fn arg(&mut self, key: &str, value: &str) {
        self.options.push(format!("{key}={value}"));
    }

    /// Helper method to add arguments to the given `Command` struct.
    ///
    /// This method checks if `input` and `output` are set, and then adds
    /// the appropriate arguments to the command. It also adds any extra
    /// options stored in `self.options`.
    ///
    /// # Parameters
    /// - `cmd`: The `Command` struct to which arguments will be added.
    fn set_args(&mut self, cmd: &mut Command) -> Result<(), DdError> {
        // If input file is specified, add it to the command as 'if' (input file).
        if let Some(input) = &self.input {
            cmd.arg(format!("if={}", input));
        } else {
            return Err(DdError::NoInput); // 'dd' requires an input file.
        }

        // If output file is specified, add it to the command as 'of' (output file).
        if let Some(output) = &self.output {
            cmd.arg(format!("of={}", output));
        }

        // Add any extra options (block size, conversion, etc.) from the options vector.
        for option in &self.options {
            cmd.arg(option);
        }

        Ok(())
    }

    /// Sets the minimum version required for the 'dd' binary.
    ///
    /// This method allows setting a specific minimum version (major, minor) for 'dd'.
    /// The version will be compared against the installed version during the `check()` method.
    ///
    /// # Parameters
    /// - `version`: A tuple representing the minimum major and minor version.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn min_version(&mut self, version: (u16, u16)) -> &mut Self {
        self.min_version = Some(version);
        self
    }

    /// Sets the input file for the 'dd' command.
    ///
    /// # Parameters
    /// - `input`: Path to the input file.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input = Some(String::from(input));
        self
    }

    /// Sets the output file for the 'dd' command.
    ///
    /// # Parameters
    /// - `output`: Path to the output file.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = Some(String::from(output));
        self
    }

    /// Sets the block size (bs) argument for the 'dd' command.
    ///
    /// The `bs` argument defines the block size for reading and writing.
    /// For example, "64K" would mean 64 kilobytes per block.
    ///
    /// # Parameters
    /// - `value`: The block size value (e.g., "64K").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn bs(&mut self, value: &str) -> &mut Self {
        self.arg("bs", value);
        self
    }

    /// Sets the conversion block size (cbs) argument for the 'dd' command.
    ///
    /// This argument specifies the size of blocks for conversion operations.
    /// It may be used alongside the `conv` argument to specify data formatting.
    pub fn cbs(&mut self, value: &str) -> &mut Self {
        self.arg("cbs", value);
        self
    }

    /// Sets the count argument for the 'dd' command.
    ///
    /// The `count` argument specifies how many blocks will be copied from the input file.
    ///
    /// # Parameters
    /// - `value`: The number of blocks to copy.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn count(&mut self, value: u64) -> &mut Self {
        self.arg("count", &value.to_string());
        self
    }

    /// Sets the seek argument for the 'dd' command.
    ///
    /// The `seek` argument skips a specified number of blocks in the output file before writing.
    ///
    /// # Parameters
    /// - `value`: The number of blocks to skip in the output file.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn seek(&mut self, value: u64) -> &mut Self {
        self.arg("seek", &value.to_string());
        self
    }

    /// Sets the skip argument for the 'dd' command.
    ///
    /// The `skip` argument skips a specified number of blocks in the input file before starting to copy.
    ///
    /// # Parameters
    /// - `value`: The number of blocks to skip in the input file.
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn skip(&mut self, value: u64) -> &mut Self {
        self.arg("skip", &value.to_string());
        self
    }

    /// Sets the status argument for the 'dd' command.
    ///
    /// The `status` argument controls the verbosity of the output. For example, it can show progress during execution.
    ///
    /// # Parameters
    /// - `value`: The desired status (e.g., "progress").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn status(&mut self, value: &str) -> &mut Self {
        self.arg("status", value);
        self
    }

    /// Sets the conversion argument (conv) for the 'dd' command.
    ///
    /// The `conv` argument specifies conversions to perform on the data as it is copied.
    /// For example, `noerror` can be used to continue reading even if errors are encountered.
    ///
    /// # Parameters
    /// - `value`: The conversion to perform (e.g., "noerror").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn conv(&mut self, value: &str) -> &mut Self {
        self.arg("conv", value);
        self
    }

    /// Sets the input block size (ibs) argument for the 'dd' command.
    ///
    /// This specifies the block size for reading from the input file.
    ///
    /// # Parameters
    /// - `value`: The input block size (e.g., "64K").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn ibs(&mut self, value: &str) -> &mut Self {
        self.arg("ibs", value);
        self
    }

    /// Sets the input flag (iflag) argument for the 'dd' command.
    ///
    /// The `iflag` argument defines input-specific flags (e.g., `fullblock` to ensure all input is read).
    ///
    /// # Parameters
    /// - `value`: The flag value (e.g., "fullblock").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn iflag(&mut self, value: &str) -> &mut Self {
        self.arg("iflag", value);
        self
    }

    /// Sets the output block size (obs) argument for the 'dd' command.
    ///
    /// This specifies the block size for writing to the output file.
    ///
    /// # Parameters
    /// - `value`: The output block size (e.g., "64K").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn obs(&mut self, value: &str) -> &mut Self {
        self.arg("obs", value);
        self
    }

    /// Sets the output flag (oflag) argument for the 'dd' command.
    ///
    /// The `oflag` argument defines output-specific flags (e.g., `append` to append data).
    ///
    /// # Parameters
    /// - `value`: The flag value (e.g., "append").
    ///
    /// # Returns
    /// - `&mut Self` to allow for method chaining.
    pub fn oflag(&mut self, value: &str) -> &mut Self {
        self.arg("oflag", value);
        self
    }

    /// Spawns and runs the 'dd' command with the configured options.
    ///
    /// This method first checks the binary, then constructs the 'dd' command
    /// with the provided input, output, and options. It returns the command's
    /// stdout output if successful.
    ///
    /// # Returns
    /// - `Ok(String)` containing the command output if the process runs successfully.
    /// - `Err(DdError)` if an error occurs at any stage.
    pub fn spawn(&mut self) -> Result<String, DdError> {
        // Check if 'dd' binary is available and valid.
        self.check()?;

        let mut cmd = Command::new(&self.binary);
        self.set_args(&mut cmd)?;

        // Execute the command.
        let output = cmd.output();

        match output {
            Ok(output) if output.status.success() => {
                // Convert stdout to String and return.
                String::from_utf8(output.stdout).map_err(|_| DdError::InvalidUTF8)
            }
            Ok(output) => Err(DdError::CantRun(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8(output.stderr)
                    .unwrap_or(String::from("unable to get stderr from 'dd'")),
            ))),
            Err(e) => Err(DdError::CantRun(e)),
        }
    }
}
