use std::collections::{BTreeMap, HashMap};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, 95437);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 1581595);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 24933642);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 1544176);
        }
    }
}

#[derive(Debug)]
struct DirEntry {
    name: String,
    dirs: HashMap<String, DirEntry>,
    files: HashMap<String, FileEntry>,
}

#[derive(Debug)]
struct FileEntry {
    size: usize,
}

enum Command {
    CdRoot,
    CdUp,
    CdDown(String),
    Ls,
}
enum LsOutput {
    Dir(String),
    File(String, usize),
}

enum TerminalEntry {
    Command(Command),
    Output(LsOutput),
}

fn parse_terminal_output(line: &str) -> TerminalEntry {
    match line.split_once(' ') {
        Some(("$", "ls")) => TerminalEntry::Command(Command::Ls),
        Some(("$", rest)) => match rest.split_once(' ') {
            Some(("cd", "/")) => TerminalEntry::Command(Command::CdRoot),
            Some(("cd", "..")) => TerminalEntry::Command(Command::CdUp),
            Some(("cd", dir_name)) => TerminalEntry::Command(Command::CdDown(dir_name.to_string())),
            None => TerminalEntry::Command(Command::Ls),
            _ => panic!(),
        },
        Some(("dir", dir_name)) => TerminalEntry::Output(LsOutput::Dir(dir_name.to_string())),
        Some((size, file_name)) => {
            TerminalEntry::Output(LsOutput::File(file_name.to_string(), size.parse().unwrap()))
        }
        _ => panic!(),
    }
}

fn parse_terminal_outputs(input: &str) -> Vec<TerminalEntry> {
    input.lines().map(parse_terminal_output).collect()
}

fn parse_fs(input: &str) -> DirEntry {
    let mut root = DirEntry {
        name: "/".to_string(),
        dirs: HashMap::new(),
        files: HashMap::new(),
    };

    let mut context: Vec<String> = vec![];

    for line in parse_terminal_outputs(input).into_iter() {
        match line {
            TerminalEntry::Command(Command::CdRoot) => {
                context = vec![];
            }
            TerminalEntry::Command(Command::CdUp) => {
                context.pop();
            }
            TerminalEntry::Command(Command::CdDown(dir_name)) => {
                context.push(dir_name);
            }
            TerminalEntry::Command(Command::Ls) => { /* do nothing really */ }
            TerminalEntry::Output(LsOutput::Dir(dir_name)) => {
                let mut curr = &mut root;
                for dir in context.iter() {
                    curr = curr.dirs.get_mut(dir).unwrap();
                }
                curr.dirs.insert(
                    dir_name.to_string(),
                    DirEntry {
                        name: dir_name.to_string(),
                        dirs: HashMap::new(),
                        files: HashMap::new(),
                    },
                );
            }
            TerminalEntry::Output(LsOutput::File(name, size)) => {
                let mut curr = &mut root;
                for dir in context.iter() {
                    curr = curr.dirs.get_mut(dir).unwrap();
                }
                curr.files
                    .insert(name.to_string(), FileEntry { size });
            }
        };
    }

    root
}

pub fn solve_part1(input: &str) -> usize {
    let root = parse_fs(input);

    let mut dir_sizes: BTreeMap<String, usize> = BTreeMap::new();

    depth_first(&root, &mut |dir: &DirEntry, path: &str| {
        dir_sizes.insert(
            path.to_string(),
            dir.files.values().map(|f| f.size).sum::<usize>()
                + dir
                    .dirs
                    .values()
                    .map(|d| dir_sizes[&format!("{}/{}", path, d.name)])
                    .sum::<usize>(),
        );
    });

    dir_sizes
        .values()
        .filter(|size| **size < 100_000)
        .sum::<usize>()
}

fn depth_first(root: &DirEntry, f: &mut impl FnMut(&DirEntry, &str)) {
    depth_first_helper(root, "", f);
}

fn depth_first_helper(root: &DirEntry, path: &str, f: &mut impl FnMut(&DirEntry, &str)) {
    for dir in root.dirs.values() {
        depth_first_helper(dir, &format!("{}/{}", path, dir.name), f);
    }
    f(root, path);
}

pub fn solve_part2(input: &str) -> usize {
    let root = parse_fs(input);
    let mut dir_sizes: BTreeMap<String, usize> = BTreeMap::new();
    depth_first(&root, &mut |dir: &DirEntry, path: &str| {
        dir_sizes.insert(
            path.to_string(),
            dir.files.values().map(|f| f.size).sum::<usize>()
                + dir
                    .dirs
                    .values()
                    .map(|d| dir_sizes[&format!("{}/{}", path, d.name)])
                    .sum::<usize>(),
        );
    });
    let missing_space = dir_sizes[""] - 40_000_000;
    let mut sorted_sizes = dir_sizes.values().collect::<Vec<_>>();
    sorted_sizes.sort();
    sorted_sizes
        .into_iter()
        .find(|size| **size >= missing_space)
        .unwrap()
        .to_owned()
}
