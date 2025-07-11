use std::env;
use std::process;

mod display;
mod git_getter;
mod history;
mod llm;
mod prompt;
fn main() {
    // Get command line arguments (excluding the program name)
    let args: Vec<String> = env::args().skip(1).collect();
    // Parse and validate arguments
    let (base, comparison) = match parse_arguments(&args) {
        Ok((base, comparison)) => (base, comparison),
        Err(error_msg) => {
            eprintln!("Error: {}", error_msg);
            eprintln!("Usage: git-prgen [base_branch] [comparison_branch]");
            eprintln!("  - No arguments: compares 'main' with 'HEAD'");
            eprintln!("  - One argument: compares <base_branch> with 'HEAD'");
            eprintln!("  - Two arguments: compares <base_branch> with <comparison_branch>");
            process::exit(1);
        }
    };

    println!("Comparing '{}' with '{}'", base, comparison);

    // Call the get_git_diff function with the provided arguments
    // and print the result or error
    let git_diff = match git_getter::get_git_diff(&base, &comparison) {
        Ok(diff) => diff,
        Err(e) => {
            eprintln!("Error getting git diff: {}", e);
            process::exit(1);
        }
    };

    if git_diff.is_empty() {
        println!(
            "No differences found between '{}' and '{}'.",
            base, comparison
        );

        process::exit(0);
    }

    let summary = prompt::ask_for_ticket_summary();
    if summary.is_empty() {
        eprintln!("No summary provided. Exiting without creating a PR.");
        process::exit(0);
    }

    match llm::generate_pr_body(&git_diff, &summary) {
        Ok(pr) => {
            println!("\n✅ Generated PR:\n\n{}", pr);
            display::copy_to_clipboard(&pr);
            display::show_output_popup(&pr);

            //get feedback also
            match llm::generate_critiques(&git_diff, &summary) {
                Ok(feedback) => {
                    display::show_output_popup(&feedback);

                    if let Ok(path) = history::save_to_history(
                        &base,
                        &comparison,
                        &summary,
                        &git_diff,
                        &pr,
                        &feedback,
                    ) {
                        println!("📁 Saved to: {}", path.display());
                    }
                }
                Err(e) => eprintln!("❌ Feedback generation failed: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Error generating PR: {}", e),
    }
}

/// Parse command line arguments and return base and comparison branches
///
/// # Arguments
/// * `args` - Vector of command line arguments
///
/// # Returns
/// * `Ok((base, comparison))` - Tuple of base and comparison branch names
/// * `Err(String)` - Error message if arguments are invalid
fn parse_arguments(args: &[String]) -> Result<(String, String), String> {
    match args.len() {
        0 => {
            // No arguments: use default values
            Ok(("main".to_string(), "HEAD".to_string()))
        }
        1 => {
            // One argument: use provided base branch, default comparison
            let base = args[0].trim();
            if base.is_empty() {
                return Err("Base branch name cannot be empty".to_string());
            }
            Ok((base.to_string(), "HEAD".to_string()))
        }
        2 => {
            // Two arguments: use both provided branches
            let base = args[0].trim();
            let comparison = args[1].trim();

            if base.is_empty() {
                return Err("Base branch name cannot be empty".to_string());
            }
            if comparison.is_empty() {
                return Err("Comparison branch name cannot be empty".to_string());
            }

            Ok((base.to_string(), comparison.to_string()))
        }
        _ => {
            // Too many arguments
            Err(format!(
                "Too many arguments provided ({}). Expected 0, 1, or 2 arguments.",
                args.len()
            ))
        }
    }
}
