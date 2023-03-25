//!  Tiny CLI utility to quickly create a Java template for solving a
//! particular Leetcode (https://leetcode.com/) problem.
//!
//1 It will:
//! 1) retrieve some metadata (problem id, title, code snippet (java)) from leetcode
//! 2) Create a solution file from tempalte: "[loc|.]/src/main/java/<pacakge_name>/Solution.java"
//! 3) Create a unit test file for solution from tempalte: "[loc|.]/src/main/java/<pacakge_name>/Solution.java"

use log::{debug, info, warn};
use std::{fs::DirBuilder, path::PathBuf};

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use cmd_lib::run_fun;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_QUESTION_ID: Regex =
        Regex::new(r##"questionId":"(?P<question_id>\d+)"##).unwrap();
    static ref REGEX_SLUG: Regex = Regex::new(r##"titleSlug":"(?P<slug>[^"]+)"##).unwrap();
    static ref REGEX_TITLE: Regex = Regex::new(r##"title":"(?P<title>[^"]+)"##).unwrap();
    static ref REGEX_SNIPPET: Regex =
        Regex::new(r##"\{"lang":"Java".*"code":"(?P<snippet>.+)"\},\{"lang":"Python""##).unwrap();
}

/// Tiny CLI utility to quickly create a Java template for solving a
/// particular Leetcode (https://leetcode.com/) problem.
#[derive(Parser, Debug)]
#[command(author, about, long_about=None)]
struct CliArgs {
    /// Leetcode problem URL to create a template Java solution/test for.
    url: String,

    /// Target root location for the created files (defaults to current dir).
    #[arg(short, long, value_name = "TARGET_LOCATION", value_hint = clap::ValueHint::DirPath)]
    loc: Option<std::path::PathBuf>,

    /// Controls logging output (-q, -v, -vv, -vvv, -vvvv). Defaults to errors only.
    #[clap(flatten)]
    verbose: Verbosity,

    /// Allows overwriting existing files
    #[clap(long, short, action)]
    force: bool,
}

type AnyError = Box<dyn std::error::Error>;

#[derive(Debug, Eq, PartialEq)]
struct TaskAttrs {
    question_id: String,
    slug: String,
    title: String,
    snippet: String,
}

impl TaskAttrs {
    pub fn new(question_id: &str, title: &str, slug: &str, snippet: &str) -> Self {
        Self {
            question_id: question_id.into(),
            slug: slug.into(),
            title: title.into(),
            snippet: snippet.into(),
        }
    }

    pub fn package_name(&self) -> String {
        format!("_{:0>4}_{}", self.question_id, &self.slug.replace('-', "_"))
    }
}

fn parse_task_attributes(input: String) -> Result<TaskAttrs, AnyError> {
    let captures = REGEX_QUESTION_ID.captures_iter(&input).next().unwrap();
    let question_id = captures.name("question_id").unwrap().as_str();

    let captures = REGEX_SLUG.captures_iter(&input).next().unwrap();
    let slug = captures.name("slug").unwrap().as_str();

    let captures = REGEX_TITLE.captures_iter(&input).next().unwrap();
    let title = captures.name("title").unwrap().as_str();

    let captures = REGEX_SNIPPET.captures_iter(&input).next().unwrap();
    let snippet_raw = captures.name("snippet").unwrap().as_str();
    let snippet = &snippet_raw.replace("\\n", "\n");

    Ok(TaskAttrs::new(question_id, title, slug, snippet))
}

fn create_solution_file(params: &TaskAttrs, cli_args: &CliArgs) -> Result<(), AnyError> {
    let location_root = cli_args.loc.clone().unwrap_or(PathBuf::new());
    info!(
        "Creating solution file at root: {}",
        location_root.to_str().unwrap()
    );
    let template_raw: String = include_str!("../template/starter.java.tmpl").into();
    let template = template_raw
        .replace("{question_id}", &params.question_id)
        .replace("{title}", &params.title)
        .replace("{package_name}", &params.package_name())
        .replace("{snippet}", &params.snippet)
        .replace("{url}", &cli_args.url);

    let path = {
        let mut p = PathBuf::new();
        p.push(&location_root);
        p.push("src/main/java");
        p.push(params.package_name());
        p.push("Solution.java");
        p
    };
    if path.exists() {
        if !cli_args.force {
            eprintln!(
                " 🙁 Unable to overwrite existing solution file (consider `-f/--force`): {}",
                &path.to_str().unwrap()
            );
            return Result::Err("Unable to create file".into());
        } else {
            warn!(
                "About to overwrite existing file {}",
                &path.to_str().unwrap()
            )
        }
    }

    DirBuilder::new()
        .recursive(true)
        .create(path.parent().unwrap())?;
    std::fs::write(&path, template)?;
    println!(" 👉 Created: {}", &path.to_str().unwrap());
    debug!("Created successfully 👍");

    Ok(())
}

fn create_test(params: &TaskAttrs, cli_args: &CliArgs) -> Result<(), AnyError> {
    let location_root = cli_args.loc.clone().unwrap_or(PathBuf::new());
    info!(
        "Creating test from at root: {}",
        location_root.to_str().unwrap()
    );
    let test_template_raw: String = include_str!("../template/test_starter.java.tmpl").into();
    let test_template = test_template_raw
        .replace("{question_id}", &params.question_id)
        .replace("{title}", &params.title)
        .replace("{package_name}", &params.package_name())
        .replace("{url}", &cli_args.url);

    let path = {
        let mut p = PathBuf::new();
        p.push(&location_root);
        p.push("src/test/java");
        p.push(params.package_name());
        p.push("SolutionTest.java");
        p
    };
    if path.exists() {
        if !cli_args.force {
            eprintln!(
                " 🙁 Unable to overwrite existing test file (consider `-f/--force`): {}",
                &path.to_str().unwrap()
            );
            return Result::Err("Unable to create file".into());
        } else {
            warn!(
                "About to overwrite existing file {}",
                &path.to_str().unwrap()
            )
        }
    }

    DirBuilder::new()
        .recursive(true)
        .create(path.parent().unwrap())?;
    std::fs::write(&path, test_template)?;
    println!(" 👉 Created: {}", &path.to_str().unwrap());
    debug!("Created successfully 👍");

    Ok(())
}

fn main() -> Result<(), AnyError> {
    let cli_args = CliArgs::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli_args.verbose.log_level_filter())
        .init();

    let url = &cli_args.url;

    let text_input = {
        info!("Parsing leetcode problem via `curl` from url: {url}");
        run_fun!(curl ${url} -s)?
    };
    let task_attrs = parse_task_attributes(text_input)?;

    create_solution_file(&task_attrs, &cli_args)?;
    create_test(&task_attrs, &cli_args)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_package_name() {
        let task_attrs = TaskAttrs::new(
            "345",
            "Some examplary title",
            "some-examplary-title",
            "java code (ignored)",
        );

        assert_eq!("_0345_some_examplary_title", task_attrs.package_name())
    }

    #[test]
    fn test_parse_task_attributes() {
        let text_input: String = String::from(include_str!("../test_input.html"));
        let actual_task_attrs = parse_task_attributes(text_input).unwrap();
        let expected_task_attrs = TaskAttrs::new(
            "83",
            "Remove Duplicates from Sorted List",
            "remove-duplicates-from-sorted-list",
            "java code (ignored)",
        );
        assert_eq!(
            expected_task_attrs.question_id,
            actual_task_attrs.question_id
        );
        assert_eq!(expected_task_attrs.title, actual_task_attrs.title);
        assert_eq!(expected_task_attrs.slug, actual_task_attrs.slug);
        assert!(actual_task_attrs.snippet.len() > 0);
    }
}
