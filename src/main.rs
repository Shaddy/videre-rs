use clap::Parser;
use colored::Colorize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::OsStr;
use std::io::BufRead;
use std::path::Path;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[clap(name = "tree")]
    Tree { mode: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tree { mode } => {
            command_tree(mode);
        }
    }
}

fn command_tree(_mode: &Option<String>) {
    let stdin = std::io::stdin();

    let lines = stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    let paths = lines.iter().map(Path::new).collect::<Vec<&Path>>();

    let mut tree: BTreeMap<&OsStr, HashSet<&OsStr>> = BTreeMap::new();

    paths.iter().for_each(|&path| {
        if let Some(parent) = path.parent() {
            let parent = parent.as_os_str();

            if let Some(stem) = path.file_name() {
                if let Some(entry) = tree.get_mut(parent) {
                    entry.insert(stem);
                } else {
                    tree.insert(parent, HashSet::from([stem]));
                }
            }
        }
    });

    for (&parent, files) in tree.iter() {
        let parent = parent.to_string_lossy();

        let levels = parent.split('/').count();

        let base_padding = "\t".repeat(levels.checked_sub(2).unwrap_or(0));

        if parent != "." {
            println!("{}", format!("{base_padding}{parent}").black().on_cyan());
        }

        let mut files: Vec<&&OsStr> = files.iter().collect();

        files.sort();

        let padding = if parent == "." { "" } else { "\t" };

        for file in files {
            let file = file.to_string_lossy();
            println!("{}", format!("{base_padding}{padding}{file}").green());
        }
    }
}
