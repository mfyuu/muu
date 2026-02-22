use crate::config::ResolvedTask;

pub fn list(tasks: &[ResolvedTask]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let max_name = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);

    for task in tasks {
        let desc = task
            .def
            .description
            .as_deref()
            .unwrap_or("");
        let source_label = format!("[{}]", task.source);
        if desc.is_empty() {
            println!("{:<width$}   {source_label}", task.name, width = max_name);
        } else {
            println!(
                "{:<width$} - {desc:<desc_width$} {source_label}",
                task.name,
                width = max_name,
                desc_width = 0,
            );
        }
    }
}
