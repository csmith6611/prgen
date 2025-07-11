use std::process::Command;

#[cfg(target_os = "macos")]
pub fn show_output_popup(pr_text: &str) {
    use std::env::temp_dir;
    use std::fs::write;

    let tmp_path = temp_dir().join("prgen_output.txt");
    let _ = write(&tmp_path, pr_text);

    let script = format!(
        "tell application \"TextEdit\"\n\
            activate\n\
            open \"{}\"\n\
        end tell",
        tmp_path.display()
    );

    let _ = Command::new("osascript").arg("-e").arg(script).status();
}

#[cfg(target_os = "linux")]
pub fn show_output_popup(pr_text: &str) {
    let _ = Command::new("zenity")
        .args([
            "--text-info",
            "--title=Generated Pull Request",
            "--width=800",
            "--height=600",
        ])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(pr_text.as_bytes())?;
            }
            child.wait()?;
            Ok(())
        });
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "macos")]
    {
        let mut pbcopy = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run pbcopy");

        if let Some(stdin) = &mut pbcopy.stdin {
            use std::io::Write;
            stdin.write_all(text.as_bytes()).unwrap();
        }
    }

    #[cfg(target_os = "linux")]
    {
        let result = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn();

        if let Ok(mut xclip) = result {
            if let Some(stdin) = &mut xclip.stdin {
                use std::io::Write;
                let _ = stdin.write_all(text.as_bytes());
            }
        } else {
            eprintln!("⚠️ xclip not found. Clipboard copy skipped.");
        }
    }
}
