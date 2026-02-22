use crate::config::ResolvedTask;

pub fn list(tasks: &[ResolvedTask]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let max_name = tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);
    let max_desc = tasks
        .iter()
        .map(|t| t.def.description.as_deref().unwrap_or("").len())
        .max()
        .unwrap_or(0);

    for task in tasks {
        let desc = task.def.description.as_deref().unwrap_or("");
        let source_label = format!("[{}]", task.source);
        if desc.is_empty() {
            println!(
                "{:<name_w$}  {:<desc_w$} \x1b[2m{source_label}\x1b[22m",
                task.name,
                "",
                name_w = max_name,
                desc_w = max_desc,
            );
        } else {
            println!(
                "{:<name_w$}  \x1b[2m{desc:<desc_w$} {source_label}\x1b[22m",
                task.name,
                name_w = max_name,
                desc_w = max_desc,
            );
        }
    }
}
