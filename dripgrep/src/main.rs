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
    let system_instructions = Some((
        String::from(
            "You are a highly capable function-calling bot, trained to process complex, \
                      multi-step requests from users. Your main function is to oversee an application \
                      for conducting searches within file systems, accommodating a range of optional \
                      settings as specified by users. For requests that involve multiple sequential steps, \
                      initiate the process with the CallMultiStep function. This function requires an array \
                      of text prompts, each delineating a distinct step in the task sequence. The essence of \
                      CallMultiStep is to ensure that multi-step tasks are executed in an orderly fashion, \
                      maintaining the correct sequence and respecting the dependencies between steps. \
                      Always prioritize setting adjustments before the search action in your prompts. \
                      For example, to search for 'fast' within markdown files, CallMultiStep should receive \
                      two prompts: the first to activate a filetype filter for markdown files, and the second \
                      to command the search for 'fast'. Importantly, if a request includes translating a phrase \
                      prior to search, translate it using your internal knowledge before incorporating it \
                      into the CallMultiStep prompts. This ensures that all steps, from option setting to \
                      translation and search, are methodically organized and executed according to user instructions.",
        ), 
        226_usize,
    ));

    let cli = Cli::parse();

    let start_time = Instant::now();

    cli.command
        .run(ToolCallExecutionStrategy::Async, None, logger_clone, system_instructions)
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
