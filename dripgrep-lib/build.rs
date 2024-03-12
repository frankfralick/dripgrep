use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // This is the path to where embeddings will be stored.
    let relative_path = PathBuf::from("../embedding/function_embeddings.bin");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let absolute_path = PathBuf::from(manifest_dir).join(relative_path);

    if let Some(parent) = absolute_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
    }

    println!(
        "cargo:warning=FUNC_ENUMS_EMBED_PATH set to: {}",
        absolute_path.display()
    );
    println!(
        "cargo:rustc-env=FUNC_ENUMS_EMBED_PATH={}",
        absolute_path.display()
    );

    let embedding_model = "text-embedding-3-small";
    println!(
        "cargo:warning=FUNC_ENUMS_EMBED_MODEL set to: {}",
        embedding_model
    );
    println!("cargo:rustc-env=FUNC_ENUMS_EMBED_MODEL={}", embedding_model);

    let max_response_tokens = 1000_u16;
    println!(
        "cargo:warning=FUNC_ENUMS_MAX_RESPONSE_TOKENS set to: {}",
        max_response_tokens
    );
    println!(
        "cargo:rustc-env=FUNC_ENUMS_MAX_RESPONSE_TOKENS={}",
        max_response_tokens
    );

    let max_request_tokens = 4191_u16;
    println!(
        "cargo:warning=FUNC_ENUMS_MAX_REQUEST_TOKENS set to: {}",
        max_request_tokens
    );
    println!(
        "cargo:rustc-env=FUNC_ENUMS_MAX_REQUEST_TOKENS={}",
        max_request_tokens
    );

    let max_func_tokens = 500_u16;
    println!(
        "cargo:warning=FUNC_ENUMS_MAX_FUNC_TOKENS set to: {}",
        max_func_tokens
    );
    println!(
        "cargo:rustc-env=FUNC_ENUMS_MAX_FUNC_TOKENS={}",
        max_func_tokens
    );

    // Wrapping something like ripgrep's officially supported file types will result in more than a
    // hundred variants, and we likely want to be able to control what we show the model in those
    // cases. This may be something that wants to be an attribute that we can make per function
    // calls on this. Can also see a situation where the top ranked function has a large number of
    // allowed options but they all need to be present for some reason. For now, this is a global
    // limit for argument enum values.
    let max_single_arg_tokens = "20";
    println!(
        "cargo:warning=FUNC_ENUMS_MAX_SINGLE_ARG_TOKENS set to: {}",
        max_single_arg_tokens
    );
    println!(
        "cargo:rustc-env=FUNC_ENUMS_MAX_SINGLE_ARG_TOKENS={}",
        max_single_arg_tokens
    );
}
