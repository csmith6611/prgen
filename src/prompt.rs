use dialoguer::Input;

pub fn ask_for_ticket_summary() -> String {
    Input::new()
        .with_prompt("Enter ticket summary")
        .interact_text()
        .unwrap_or_else(|_| "No summary provided".to_string())
}
