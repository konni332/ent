

use clap::Parser;
use anyhow::Result;
use ent_tree::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    execute(cli)?;
    Ok(())
}



fn execute(cli: Cli) -> Result<()> {
    match &cli.depth {
        Some(depth) => {
            if *depth <= 0 {
                return Err(anyhow::anyhow!("Depth must be greater than 0"));
            }
        },
        None => {}
    }
    let cwd = std::env::current_dir()?.to_string_lossy().into_owned();
    let path = match &cli.path {
        Some(path) => path,
        None => {
            &cwd
        }
    };
    let tree = TreeEntry::build(&cli, &path)?;
    print_tree_root(&tree);

    match cli.export {
        None => {},
        Some(format) => {
            let export_path = std::env::current_dir()?.join("ent-tree.json");
            tree.export(export_path, format)?;
        }
    }

    Ok(())
}