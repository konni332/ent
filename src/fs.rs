use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use anyhow::{bail, Result};
use serde::Serialize;
use crate::cli::{Cli, Format};

#[derive(Debug, Serialize, Clone)]
pub struct TreeEntry {
    pub name: String,
    pub is_dir: bool,
    pub children: Option<Vec<TreeEntry>>,
}

impl TreeEntry {
    pub fn new(name: String, is_dir: bool) -> Self {
        Self {
            name,
            is_dir,
            children: None,
        }
    }
    pub fn new_file(name: String) -> Self {
        Self::new(name, false)
    }
    pub fn new_dir(name: String) -> Self {
        let mut new = Self::new(name, true);
        new.children = Some(Vec::new());
        new
    }

    pub fn build(cli: &Cli, path: &str) -> Result<TreeEntry> {
        let path = PathBuf::from(path);
        if !path.exists() {
            bail!("Path does not exist: {}", path.display());
        }

        let root_name = path.file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string());

        let mut root = TreeEntry::new_dir(root_name);
        let depth = 1;
        Self::build_recursive(cli, &path, &mut root, depth)?;
        Ok(root)

    }

    fn build_recursive(cli: &Cli, path: &Path, parent: &mut TreeEntry, depth: usize) -> Result<()> {
        let walker = WalkBuilder::new(path)
            .hidden(!cli.hidden && !cli.all)
            .git_ignore(!cli.ignored && !cli.all)
            .git_exclude(!cli.ignored && !cli.all)
            .parents(true)
            .max_depth(Some(1))
            .build();

        match &cli.depth {
            Some(d) if d < &depth => return Ok(()),
            _ => (),
        }

        for result in walker {
            let entry = match result {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error walking directory: {}", e);
                    continue;
                }
            };
            if entry.path() == path {
                continue;
            }
            let path = entry.path();
            let name = path.file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();

            if !cli.hidden && is_hidden(&path) {
                continue;
            }

            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

            if cli.dirs_only && !is_dir {
                continue;
            }
            if cli.files_only && is_dir {
                continue;
            }
            let mut child: TreeEntry = if is_dir {
                TreeEntry::new_dir(name)
            }
            else {
                TreeEntry::new_file(name)
            };

            if is_dir {
                let new_depth = depth + 1;
                Self::build_recursive(cli, &path, &mut child, new_depth)?;
            }

            if let Some(children) = &mut parent.children {
                children.push(child);
            }
        }

        Ok(())
    }

    pub fn export<P: AsRef<Path>>(&self, path: P, format: Format) -> Result<()> {
        match format {
            Format::Json => {
                let export_str = serde_json::to_string_pretty(self)?;
                std::fs::write(path, export_str)?;
            },
        }
        Ok(())
    }
}

pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .map(|s| s.to_string_lossy().starts_with('.'))
        .unwrap_or(false)
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{create_dir, File};
    use std::io::Write;

    #[test]
    fn test_new_file() {
        let file = TreeEntry::new_file("test.txt".to_string());
        assert_eq!(file.name, "test.txt");
        assert!(!file.is_dir);
        assert!(file.children.is_none());
    }

    #[test]
    fn test_new_dir() {
        let dir = TreeEntry::new_dir("src".to_string());
        assert_eq!(dir.name, "src");
        assert!(dir.is_dir);
        assert!(dir.children.is_some());
    }

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(&PathBuf::from(".hidden")));
        assert!(!is_hidden(&PathBuf::from("visible")));
    }

    #[test]
    fn test_build_simple_tree() -> Result<()> {
        let dir = tempdir()?;
        let root_path = dir.path();

        // Create structure:
        // root/
        // ├── file.txt
        // └── subdir/
        //     └── nested.txt
        File::create(root_path.join("file.txt"))?.write_all(b"Hello")?;
        create_dir(root_path.join("subdir"))?;
        File::create(root_path.join("subdir/nested.txt"))?;

        let cli = Cli {
            hidden: false,
            all: false,
            ignored: false,
            depth: None,
            dirs_only: false,
            files_only: false,
            ..Default::default()
        };

        let tree = TreeEntry::build(&cli, root_path.to_str().unwrap())?;
        assert_eq!(tree.is_dir, true);
        assert!(tree.children.as_ref().unwrap().iter().any(|e| e.name == "file.txt"));
        assert!(tree.children.as_ref().unwrap().iter().any(|e| e.name == "subdir"));

        Ok(())
    }

    #[test]
    fn test_export_json() -> Result<()> {
        let tree = TreeEntry::new_dir("project".to_string());
        let tmp_file = tempfile::NamedTempFile::new()?;

        tree.export(tmp_file.path(), Format::Json)?;
        let contents = std::fs::read_to_string(tmp_file.path())?;
        assert!(contents.contains("\"name\": \"project\""));
        Ok(())
    }
}



















