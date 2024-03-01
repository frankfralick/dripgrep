use clap::Parser;
use dripgrep_lib::cli::cli_entry::Cli;
use openai_func_enums::{logger_task, CommandError, Logger, RunCommand, ToolCallExecutionStrategy};
use std::sync::Arc;
use std::time::Instant;
use tokio::spawn;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel(100);
    let logger = Arc::new(Logger { sender });
    spawn(logger_task(receiver));
    let logger_clone = logger.clone();

    let cli = Cli::parse();

    let start_time = Instant::now();

    cli.command
        .run(ToolCallExecutionStrategy::Async, None, logger_clone)
        .await
        .map_err(|e| {
            Box::new(CommandError::new(&format!(
                "Command failed with error: {}",
                e
            )))
        })?;

    let duration = start_time.elapsed();
    println!("Command completed in {:.2} seconds", duration.as_secs_f64());

    Ok(())
}
