use super::{
    args::{
        CaseSensitivity, Enabled, FileDisplayMode, FileType, Restriction, SortBy, SortOrdering,
    },
    command_funcs::*,
};
use clap::Subcommand;
use openai_func_enums::{Logger, RunCommand, ToolCallExecutionStrategy, ToolSet};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Subcommand, ToolSet)]
pub enum Commands {
    /// Sets case sensitivity
    CaseFilter {
        case_sensitivity: CaseSensitivity,
    },

    /// Sets how many of the lines of the surrounding context to display
    ContextLines {
        lines_before: i32,
        lines_after: i32,
    },

    /// Sets wether to treat CRLF ('\r\n') as a line terminator instead of just '\n'
    CRLF {
        enabled: Enabled,
    },

    /// Sets whether or not regex patterns using '.' match newline characters
    DotAll {
        enabled: Enabled,
    },

    /// Sets display for files. Currently included, files with a match, or files without match
    Files {
        file_display_mode: FileDisplayMode,
    },

    /// Sets whether to include or exclude a filetype
    FileTypeFilter {
        enabled: Enabled,
        file_type: FileType,
    },

    /// Treat the pattern as a literal string
    FixedStrings {
        enabled: Enabled,
    },

    /// Sets whether to include compressed files
    IncludeZip {
        enabled: Enabled,
    },

    /// Sets whether to enable multiline mode
    Multiline {
        enabled: Enabled,
    },

    /// Replace every match with the text given when printing results
    Replace {
        replacement_text: String,
    },

    /// Sets whether to search in ignored, hidden, and binary files
    RestrictionLevel {
        restriction: Restriction,
    },

    /// Sorts results by the given method
    SortResultsBy {
        sort_by: SortBy,
        sort_ordering: SortOrdering,
    },

    /// Sets whether statistics should be included or not
    Statistics {
        enabled: Enabled,
    },

    /// Sets the number of threads to use
    Threads {
        thread_count: i32,
    },

    /// Sets whether to treat binary files as if they were text
    TreatBinaryAsText {
        enabled: Enabled,
    },

    /// Sets whether to trim whitespace at the beginning and end of lines
    TrimWhitespace {
        enabled: Enabled,
    },

    /// Searches for a pattern
    Search {
        pattern: String,
    },

    /// CallMultiStep is designed to efficiently process complex, multi-step user requests. It takes an array of text prompts, each detailing a specific step in a sequential task. This function is crucial for handling requests where the output of one step forms the input of the next. When constructing the prompt list, consider the dependency and order of tasks. Independent tasks within the same step should be consolidated into a single prompt to leverage parallel processing capabilities. This function ensures that multi-step tasks are executed in the correct sequence and that all dependencies are respected, thus faithfully representing and fulfilling the user's request."
    CallMultiStep {
        prompt_list: Vec<String>,
    },

    GPT {
        prompt: String,
    },
}

#[async_trait]
impl RunCommand for Commands {
    async fn run(
        &self,
        execution_strategy: ToolCallExecutionStrategy,
        arguments: Option<Vec<String>>,
        logger: Arc<Logger>,
    ) -> Result<
        (Option<String>, Option<Vec<String>>),
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        let model_name = "gpt-4-1106-preview";

        let system_message = "You are a highly capable function-calling bot, trained to process complex, \
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
                      translation and search, are methodically organized and executed according to user instructions.";

        match self {
            Commands::CaseFilter { case_sensitivity } => {
                return case_filter(arguments, case_sensitivity).await;
            }

            Commands::ContextLines {
                lines_before,
                lines_after,
            } => {
                return context_lines(arguments, lines_before, lines_after).await;
            }

            Commands::CRLF { enabled } => {
                return crlf(arguments, enabled).await;
            }

            Commands::DotAll { enabled } => {
                return dot_all(arguments, enabled).await;
            }

            Commands::FileTypeFilter { enabled, file_type } => {
                return file_type_filter(arguments, enabled, file_type).await;
            }

            Commands::Files { file_display_mode } => {
                return files(arguments, file_display_mode).await;
            }

            Commands::FixedStrings { enabled } => {
                return fixed_strings(arguments, enabled).await;
            }

            Commands::Multiline { enabled } => {
                return multiline(arguments, enabled).await;
            }

            Commands::IncludeZip { enabled } => {
                return include_zip(arguments, enabled).await;
            }

            Commands::Replace { replacement_text } => {
                return replace(arguments, replacement_text).await;
            }

            Commands::RestrictionLevel { restriction } => {
                return restriction_level(arguments, restriction).await;
            }

            Commands::SortResultsBy {
                sort_by,
                sort_ordering,
            } => {
                return sort_results_by(arguments, sort_by, sort_ordering).await;
            }

            Commands::Statistics { enabled } => {
                return statistics(arguments, enabled).await;
            }

            Commands::Threads { thread_count } => {
                return threads(arguments, thread_count).await;
            }

            Commands::TreatBinaryAsText { enabled } => {
                return treat_binary_as_text(arguments, enabled).await;
            }

            Commands::TrimWhitespace { enabled } => {
                return trim_whitespace(arguments, enabled).await;
            }

            Commands::Search { pattern } => {
                let logger_clone = logger.clone();
                return search(arguments, logger_clone, pattern).await;
            }

            Commands::CallMultiStep { prompt_list } => {
                let logger_clone = logger.clone();

                return call_multi_step(
                    execution_strategy.clone(),
                    logger_clone,
                    model_name,
                    Some(system_message.to_string()),
                    prompt_list,
                )
                .await;
            }

            Commands::GPT { prompt } => {
                let logger_clone = logger.clone();

                return gpt(
                    execution_strategy.clone(),
                    logger_clone,
                    model_name,
                    Some(system_message.to_string()),
                    prompt,
                )
                .await;
            }
        };
    }
}
