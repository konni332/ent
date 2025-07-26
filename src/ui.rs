use std::path::Path;
use colored::Colorize;
use crate::fs::{is_hidden, TreeEntry};


pub fn print_tree_root(entry: &TreeEntry) {
    let name_str = format_entry(entry, false);
    println!("{}", name_str);
    if let Some(children) = &entry.children {
        let len = children.len();
        for (i, child) in children.iter().enumerate() {
            let hidden = is_hidden(Path::new(child.name.as_str()));
            print_tree(child, "", i == len - 1, hidden);
        }
    }
}

fn print_tree(entry: &TreeEntry, prefix: &str, is_last: bool, hidden: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let entry_str = format_entry(entry, hidden);
    println!("{}{}{}", prefix, connector, entry_str);

    if let Some(children) = &entry.children {
        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        let len = children.len();
        for (i, child) in children.iter().enumerate() {
            let hidden = is_hidden(Path::new(child.name.as_str())) || hidden;
            print_tree(child, &new_prefix, i == len - 1, hidden);
        }
    }
}

fn format_entry(entry: &TreeEntry, hidden: bool) -> String {
    if entry.is_dir {
        let name_str = format!("{}{}", " ".cyan(), entry.name.cyan());
        if hidden {
            return format!("{}{}", name_str.dimmed(),"/".cyan().dimmed());
        }
        return format!("{}{}", name_str.bold() ,"/".cyan().bold());
    }
    if hidden {
        return format!("{}{}", " ".dimmed(), entry.name.dimmed());
    }
    format!("{}{}", " ".bold(), entry.name.bold())
}
