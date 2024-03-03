use super::{
    args::{
        CaseSensitivity, Enabled, FileDisplayMode, FileType, Restriction, SortBy, SortOrdering,
    },
    commands::{
        CommandsGPT, FUNC_ENUMS_EMBED_MODEL, FUNC_ENUMS_EMBED_PATH, FUNC_ENUMS_MAX_REQUEST_TOKENS,
        FUNC_ENUMS_MAX_RESPONSE_TOKENS,
    },
};
use openai_func_enums::{
    get_ranked_function_names, single_embedding, Logger, ToolCallExecutionStrategy,
};
use std::path::Path;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;

/// Prepares a command-line argument list with case-sensitivity settings.
///
/// This asynchronous function takes optional command-line arguments and a case sensitivity setting,
/// then appends the appropriate flag for case sensitivity to the arguments. It returns a result containing
/// a success message and the possibly modified list of arguments, or an error if one occurs.
///
/// # Parameters
/// - `arguments`: An `Option<Vec<String>>` representing command-line arguments. If `Some`, the function
/// extends this list with a case sensitivity flag; if `None`, it simply adds the case sensitivity flag
/// to an empty list.
/// - `case_sensitivity`: A reference to an enum representing the case sensitivity setting. The enum
///   `CaseSensitivity` has variants for ignoring case, being case-sensitive, and employing smart case logic.
///   The "SmartCase" variant will treat search patters as case-insensitive, unless a capital
///   character is present.
///  
///
/// # Returns
/// A `Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>`. On success,
/// it returns a tuple containing an `Option<String>` with a success message and an `Option<Vec<String>>` with the
/// possibly extended list of command-line arguments including the case sensitivity flag. On failure, it returns
/// an error boxed to allow for any type of error that implements the `std::error::Error` trait, along with `Send`,
/// `Sync`, and a static lifetime.
///
/// # Behavior
/// - The function first checks if the `arguments` parameter is `Some` or `None`. If `Some`, it clones the vector
/// of arguments provided; if `None`, it initializes an empty vector.
/// - Based on the `case_sensitivity` parameter, the function appends a specific flag to the arguments vector:
///     - For `CaseSensitivity::IgnoreCase`, it appends `--ignore-case`.
///     - For `CaseSensitivity::CaseSensitive`, it appends `--case-sensitive`.
///     - For `CaseSensitivity::SmartCase`, it appends `--smart-case`.
/// - After processing, the function constructs a success message and returns both the message and the updated
/// list of arguments within an `Ok` variant of the `Result`.
///
/// # Example Usage
/// ```
/// async fn example_usage() {
///     let arguments = Some(vec![String::from("--pretty", "--type", "rust")]);
///     let case_sensitivity = CaseSensitivity::IgnoreCase;
///
///     match case_filter(arguments, &case_sensitivity).await {
///         Ok((Some(result), Some(args))) => {
///             println!("Result: {}", result);
///             println!("Arguments: {:?}", args);
///         }
///         Err(e) => println!("An error occurred: {}", e),
///         _ => println!("Unexpected result."),
///     }
/// }
/// ```
///
/// Note: Ensure that the `CaseSensitivity` enum is defined in your codebase with the variants `IgnoreCase`,
/// `CaseSensitive`, and `SmartCase` as used by the `case_filter` function. The function is asynchronous and
/// must be awaited when called.
pub async fn case_filter(
    arguments: Option<Vec<String>>,
    case_sensitivity: &CaseSensitivity,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match case_sensitivity {
        CaseSensitivity::IgnoreCase => args.push(String::from("--ignore-case")),
        CaseSensitivity::CaseSensitive => args.push(String::from("--case-sensitive")),
        CaseSensitivity::SmartCase => args.push(String::from("--smart-case")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn context_lines(
    arguments: Option<Vec<String>>,
    lines_before: &i32,
    lines_after: &i32,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    args.push(String::from("--before_context"));
    args.push(lines_before.to_string());
    args.push(String::from("--after-context"));
    args.push(lines_after.to_string());

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn crlf(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--crlf")),
        Enabled::No => args.push(String::from("--no-crlf")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn debug(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args = arguments.unwrap_or_else(Vec::new);

    if let Enabled::Yes = enabled {
        args.push(String::from("--debug"));
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn dot_all(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => {
            let multiline = String::from("--multiline");
            if !args.iter().any(|p| p == &multiline) {
                args.push(multiline);
            }
            args.push(String::from("--multiline-dotall"));
        }
        Enabled::No => args.push(String::from("--no-multiline-dotall")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn file_type_filter(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
    file_type: &FileType,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--type")),
        Enabled::No => args.push(String::from("--type-not")),
    }

    args.push(file_type.flag_value());

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn files(
    arguments: Option<Vec<String>>,
    file_display_mode: &FileDisplayMode,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match file_display_mode {
        FileDisplayMode::FilesIncluded => {
            args.push(String::from("--files"));
            args.push(String::from("."));
        }
        FileDisplayMode::FilesWithMatch => args.push(String::from("--files-with-matches")),
        FileDisplayMode::FilesWithoutMatch => args.push(String::from("--files-without-match")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn fixed_strings(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--fixed-strings")),
        Enabled::No => args.push(String::from("--no-fixed-strings")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn multiline(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--multiline")),
        Enabled::No => args.push(String::from("--no-multiline")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn include_zip(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--search-zip")),
        Enabled::No => args.push(String::from("--no-search-zip")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn replace(
    arguments: Option<Vec<String>>,
    replacement_text: &String,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    args.push(String::from("--replace"));
    let quoted_replacement_text = format!("\"{}\"", replacement_text);
    // args.push(replacement_text.to_string());
    args.push(quoted_replacement_text);

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn restriction_level(
    arguments: Option<Vec<String>>,
    restriction: &Restriction,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match restriction {
        Restriction::Default => {}
        Restriction::IncludeIgnored => args.push(String::from("-u")),
        Restriction::IncludeIgnoredHidden => args.push(String::from("-uu")),
        Restriction::IncludeIgnoredHiddenBinary => args.push(String::from("-uuu")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn sort_results_by(
    arguments: Option<Vec<String>>,
    sort_by: &SortBy,
    sort_ordering: &SortOrdering,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match sort_ordering {
        SortOrdering::Ascending => args.push(String::from("--sort")),
        SortOrdering::Descending => args.push(String::from("--sortr")),
    }

    args.push(sort_by.flag_value());

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn statistics(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--stats")),
        Enabled::No => args.push(String::from("--no-stats")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn threads(
    arguments: Option<Vec<String>>,
    thread_count: &i32,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    // If thread_count is zero we don't need to do anything.
    if thread_count > &0_i32 {
        args.push(String::from("--threads"));
        args.push(thread_count.to_string());
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn treat_binary_as_text(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--text")),
        Enabled::No => args.push(String::from("--no-text")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn trim_whitespace(
    arguments: Option<Vec<String>>,
    enabled: &Enabled,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut args: Vec<String> = vec![];
    if let Some(arguments) = arguments {
        args.extend(arguments.to_vec());
    }

    match enabled {
        Enabled::Yes => args.push(String::from("--trim")),
        Enabled::No => args.push(String::from("--no-trim")),
    }

    let result = String::from("Ok.");
    Ok((Some(result), Some(args)))
}

pub async fn search(
    arguments: Option<Vec<String>>,
    logger: Arc<Logger>,
    pattern: &String,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let mut command = Command::new("rg");
    command.arg(pattern);
    command.arg("--pretty");

    if let Some(arguments) = arguments {
        for arg in arguments {
            command.arg(arg);
        }
    }

    let _ = logger
        .sender
        .send(String::from("\nExecuting command with args:\n"))
        .await;
    let message = format!("{:#?}", command);

    // let mut arg_out = vec![String::from("rg")];
    // arg_out.extend(
    //     command
    //         .as_std()
    //         .get_args()
    //         .map(|arg| arg.to_string_lossy().into_owned())
    //         .collect::<Vec<_>>(),
    // );

    // let message = arg_out.join(" ");

    let _ = logger.sender.send(message).await;
    let _ = logger.sender.send(String::from("")).await;

    let output = command.output().await.expect("Failed to execute command");

    if output.status.success() {
        let stdout = std::str::from_utf8(&output.stdout).expect("Failed to parse stdout as UTF-8");
        let log_message = format!("Found matches:\n{}", stdout);
        let _ = logger.sender.send(log_message).await;
    } else {
        // There is some sort of error but there are still likely results
        let stderr = std::str::from_utf8(&output.stderr).expect("Failed to parse stderr as UTF-8");
        let log_message = format!("Command failed with error:\n{}", stderr);
        let _ = logger.sender.send(log_message).await;

        let stdout = std::str::from_utf8(&output.stdout).expect("Failed to parse stdout as UTF-8");
        let log_message = format!("Found matches:\n{}", stdout);
        let _ = logger.sender.send(log_message).await;
    }

    let result = String::from("Ok.");
    Ok((Some(result), None))
}

pub async fn call_multi_step(
    execution_strategy: ToolCallExecutionStrategy,
    logger: Arc<Logger>,
    model_name: &str,
    system_meessage: Option<String>,
    prompt_list: &Vec<String>,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = logger
        .sender
        .send(String::from("\nMulti-step prompt list:\n"))
        .await;
    let message = format!("{:#?}", prompt_list);
    let _ = logger.sender.send(message).await;

    let prior_result = Arc::new(Mutex::new(None));

    let command_args_list: Vec<String> = Vec::new();
    let command_args = Arc::new(Mutex::new(Some(command_args_list)));
    for (i, prompt) in prompt_list.iter().enumerate() {
        let prior_result_clone = prior_result.clone();
        let command_args_clone = command_args.clone();
        let logger_clone = logger.clone();

        let prompt_embedding = single_embedding(prompt, FUNC_ENUMS_EMBED_MODEL).await?;
        let embed_path = Path::new(FUNC_ENUMS_EMBED_PATH);
        let ranked_func_names = get_ranked_function_names(prompt_embedding, embed_path).await?;

        let required_func_names = vec![String::from("CallMultiStep")];

        match i {
            0 => {
                CommandsGPT::run(
                    &prompt.to_string(),
                    model_name,
                    FUNC_ENUMS_MAX_REQUEST_TOKENS.into(),
                    FUNC_ENUMS_MAX_RESPONSE_TOKENS,
                    system_meessage.clone(),
                    prior_result_clone,
                    execution_strategy.clone(),
                    command_args_clone,
                    Some(ranked_func_names),
                    Some(required_func_names),
                    logger_clone,
                )
                .await?
            }

            _ => {
                let prior_result_guard = prior_result.lock().await;
                if let Some(prior) = &*prior_result_guard {
                    let new_prompt = format!("The prior result was: {}. {}", prior.clone(), prompt);
                    drop(prior_result_guard);

                    CommandsGPT::run(
                        &new_prompt,
                        model_name,
                        FUNC_ENUMS_MAX_REQUEST_TOKENS.into(),
                        FUNC_ENUMS_MAX_RESPONSE_TOKENS,
                        system_meessage.clone(),
                        prior_result_clone,
                        execution_strategy.clone(),
                        command_args_clone,
                        None,
                        None,
                        logger_clone,
                    )
                    .await?
                } else {
                    *prior_result.lock().await = None;
                }
            }
        }
    }

    let result = String::from("Ok.");
    Ok((Some(result), None))
}

pub async fn gpt(
    execution_strategy: ToolCallExecutionStrategy,
    logger: Arc<Logger>,
    model_name: &str,
    system_message: Option<String>,
    prompt: &String,
) -> Result<(Option<String>, Option<Vec<String>>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let prompt_embedding = single_embedding(prompt, FUNC_ENUMS_EMBED_MODEL).await?;

    let prior_result = Arc::new(Mutex::new(None));
    let command_args = Arc::new(Mutex::new(None));
    let logger_clone = logger.clone();

    let embed_path = Path::new(FUNC_ENUMS_EMBED_PATH);

    let ranked_func_names = get_ranked_function_names(prompt_embedding, embed_path).await?;
    let required_func_names = vec![String::from("CallMultiStep")];

    CommandsGPT::run(
        prompt,
        model_name,
        FUNC_ENUMS_MAX_REQUEST_TOKENS.into(),
        FUNC_ENUMS_MAX_RESPONSE_TOKENS,
        system_message.clone(),
        prior_result,
        execution_strategy.clone(),
        command_args,
        Some(ranked_func_names),
        Some(required_func_names),
        logger_clone,
    )
    .await?;

    let result = String::from("Ok.");
    Ok((Some(result), None))
}
