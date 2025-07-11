use std::io::stdin;

mod git;

fn main() {
    let args = stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    //check if second argument is provided, if not, use "HEAD" as default

    if args.len() > 2 {
        eprintln!("Usage: git-prgen <base_branch> <comparison_branch>");
        return;
    }

    let comparison: &str = if args.len() > 1 { &args[1] } else { "HEAD" };

    // Call the get_git_diff function with the provided arguments
    // and print the result or error

    match git::get_git_diff(&args[0], comparison) {
        Ok(diff) => println!("Git Diff:\n{}", diff),
        Err(e) => eprintln!("Error getting git diff: {}", e),
    }
}
