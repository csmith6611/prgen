use chrono::Local;
use std::env;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub fn save_to_history(
    base: &str,
    compare: &str,
    summary: &str,
    diff: &str,
    pr: &str,
    feedback: &str,
) -> std::io::Result<PathBuf> {
    let home = env::var("HOME").expect("Cannot find HOME environment variable");
    let dir = PathBuf::from(format!("{}/.git-prgen-history", home));
    create_dir_all(&dir)?;

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let file_path = dir.join(format!("{}_{}.md", base, timestamp));

    let short_diff = &diff[..diff.len().min(100)];

    let contents = format!(
        "# PRGen Run\n\
         **Base:** `{}` → **Compare:** `{}`\n\
         **Summary:** {}\n\n\
         ## Diff (truncated)\n```\n{}\n```\n\
         ## PR Body\n{}\n\n\
         ## Feedback\n{}\n",
        base, compare, summary, short_diff, pr, feedback
    );

    write(&file_path, contents)?;

    Ok(file_path)
}
