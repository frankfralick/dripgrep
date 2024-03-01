use clap::ValueEnum;
use openai_func_enums::{arg_description, EnumDescriptor, VariantDescriptors};
use serde::Deserialize;

// TODO: Need to implement similarity search/culling arg enums with too many allowed options.
#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(description = "The supported filters for file types.")]
pub enum FileType {
    C,
    CPP,
    Config,
    CSharp,
    Go,
    Java,
    JavaScript,
    Json,
    JsonLines,
    Jupyter,
    License,
    Log,
    Lua,
    Markdown,
    OCaml,
    Org,
    PDF,
    Python,
    Rust,
    RustBuildDependency,
    Shell,
    SQL,
    Text,
    Toml,
    TypeScript,
    XML,
    Vim,
    Yaml,
    Zig,
}

impl FileType {
    pub fn flag_value(&self) -> String {
        match *self {
            FileType::C => String::from("c"),
            FileType::CPP => String::from("cpp"),
            FileType::Config => String::from("config"),
            FileType::CSharp => String::from("cs"),
            FileType::Go => String::from("go"),
            FileType::Java => String::from("java"),
            FileType::JavaScript => String::from("js"),
            FileType::Json => String::from("json"),
            FileType::JsonLines => String::from("jsonl"),
            FileType::Jupyter => String::from("jupyter"),
            FileType::License => String::from("license"),
            FileType::Log => String::from("log"),
            FileType::Lua => String::from("lua"),
            FileType::Markdown => String::from("markdown"),
            FileType::OCaml => String::from("ocaml"),
            FileType::Org => String::from("org"),
            FileType::PDF => String::from("pdf"),
            FileType::Python => String::from("py"),
            FileType::Rust => String::from("rust"),
            FileType::RustBuildDependency => String::from("d"),
            FileType::Shell => String::from("sh"),
            FileType::SQL => String::from("sql"),
            FileType::Text => String::from("txt"),
            FileType::Toml => String::from("toml"),
            FileType::TypeScript => String::from("ts"),
            FileType::XML => String::from("xml"),
            FileType::Vim => String::from("vim"),
            FileType::Yaml => String::from("yaml"),
            FileType::Zig => String::from("zig"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(description = "The supported ways to sort search results.")]
pub enum SortBy {
    None,
    Path,
    Modified,
    Accessed,
    Created,
}

impl SortBy {
    pub fn flag_value(&self) -> String {
        match *self {
            SortBy::None => String::from("none"),
            SortBy::Path => String::from("path"),
            SortBy::Modified => String::from("modified"),
            SortBy::Accessed => String::from("accessed"),
            SortBy::Created => String::from("created"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(description = "The order in which to sort results.")]
pub enum SortOrdering {
    Ascending,
    Descending,
}

// This is just a convenience. Right boolean arguments to functions aren't supported
#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(description = "Whether the setting should be enabled or not.")]
pub enum Enabled {
    Yes,
    No,
}

#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(description = "The ways to treat case sensitivity.")]
pub enum CaseSensitivity {
    IgnoreCase,
    CaseSensitive,
    SmartCase,
}

#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(
    description = "Specify search restrictions. Search in ignored, ignored and hidden, or ignored, hidden, and binary files."
)]
pub enum Restriction {
    Default,
    IncludeIgnored,
    IncludeIgnoredHidden,
    IncludeIgnoredHiddenBinary,
}

#[derive(Clone, Debug, Deserialize, EnumDescriptor, VariantDescriptors, ValueEnum)]
#[arg_description(
    description = "Specify how file paths will print. Show files included in search, files with at least one match, or files with no matches."
)]
pub enum FileDisplayMode {
    FilesIncluded,
    FilesWithMatch,
    FilesWithoutMatch,
}
