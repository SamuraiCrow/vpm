use anyhow::{Context, Result};
use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use fastrand;
use crate::cmd::{Execute, Sim};

impl Execute for Sim {
    async fn execute(&self) -> Result<()> {
        // Compile the Verilog files using Icarus Verilog
        let output_path = self.compile_verilog()?;

        // Run the simulation
        self.run_simulation(&output_path)?;

        Ok(())
    }
}

impl Sim {
    pub fn compile_verilog(&self) -> Result<PathBuf> {
        println!("Compiling Verilog files...");

        // Generate a random string for the output name
        let random_output_name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
            .collect();

        // Prepare the command to compile the Verilog files
        let mut command = Command::new("iverilog");
        command.arg("-o").arg(&random_output_name);

        // Add each Verilog file to the command
        for file in &self.verilog_files {
            command.arg(file);
        }

        // Execute the command
        let status = command.status().context("Failed to execute Icarus Verilog compilation")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to compile Verilog files."));
        }

        let output_path = Path::new(&random_output_name);

        // Check if the output binary exists
        if !output_path.exists() {
            return Err(anyhow::anyhow!("Output binary not found: {:?}", output_path));
        }

        // println!("Verilog files compiled successfully.");
        // println!("Output binary created: {:?}", output_path);
        Ok(output_path.to_path_buf())
    }

    pub fn run_simulation(&self, output_path: &PathBuf) -> Result<()> {
        println!("Running simulation...");
    
        // Print the current working directory
        let current_dir = env::current_dir().expect("Failed to get current directory");
        
        // Create the binary path by concatenating the current directory with the output name
        let binary_path: PathBuf = current_dir.join(output_path); // Trim quotes if any
        // println!("Attempting to execute binary at: {:?}", binary_path);
    
        // Execute the simulation binary generated by Icarus Verilog
        let status = Command::new(&binary_path)
            .status()
            .context("Failed to execute simulation")?;
    
        if !status.success() {
            eprintln!("Simulation failed.");
            return Ok(());
        }
    
        println!("Simulation completed successfully.");
        let _ = fs::remove_file(&binary_path);
        Ok(())
    }
}
