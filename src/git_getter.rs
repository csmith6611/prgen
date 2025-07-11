use std::io::{self};
use std::process::{Command, Stdio};

pub fn get_git_diff(base_branch: &str, comparison_branch: &str) -> io::Result<String> {
    let formatted_base = base_branch.replace(" ", "");
    let formatted_comparison = comparison_branch.replace(" ", "");
    let full_comparison_arg = format!("{}...{}", formatted_base, formatted_comparison);

    let output = Command::new("git")
        .args(["diff", &full_comparison_arg])
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Git diff failed with status: {}\n{}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}
