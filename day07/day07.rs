use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::iter::Iterator;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Node {
    File(usize),
    Dir(HashMap<String, Node>),
}

fn is_command_line(s: &str) -> bool {
    s.starts_with("$")
}

impl Node {
    fn from_command_lines<I>(it: &mut Peekable<I>) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut root_dir = Self::new_dir();

        let mut path: Vec<String> = vec![];

        let mut current_dir = &mut root_dir;

        while let Some(cmd) = it.next() {
            assert!(is_command_line(&cmd));

            match cmd.split(' ').skip(1).collect::<Vec<&str>>()[..] {
                ["cd", "/"] => {
                    path.clear();
                    current_dir = &mut root_dir;
                }
                ["cd", ".."] => {
                    path.pop().unwrap();
                    current_dir = root_dir.find_path_mut(&path.as_slice()).unwrap();
                }
                ["cd", dir] => {
                    match current_dir.entries().unwrap().get(dir) {
                        Some(_) => {}
                        None => {
                            current_dir
                                .entries_mut()
                                .unwrap()
                                .insert(dir.to_owned(), Self::new_dir());
                        }
                    };
                    path.push(dir.to_owned());
                    current_dir = current_dir.entries_mut().unwrap().get_mut(dir).unwrap();
                }
                ["ls"] => {
                    while it.peek().is_some() && !is_command_line(it.peek().unwrap()) {
                        let line = it.next().unwrap();
                        let (size_or_dir, name) = line.split_once(' ').unwrap();

                        match size_or_dir {
                            "dir" => {
                                if let None = current_dir.entries_mut().unwrap().get(name) {
                                    current_dir
                                        .entries_mut()
                                        .unwrap()
                                        .insert(name.to_owned(), Self::new_dir());
                                }
                            }
                            size_str => {
                                if let Ok(size) = size_str.parse::<usize>() {
                                    let foo = current_dir
                                        .entries_mut()
                                        .unwrap()
                                        .insert(name.to_owned(), Self::File(size));

                                    if let Some(f) = foo {
                                        println!("{} {:?} {:?}", name, f, Self::File(size));
                                        assert!(f == Self::File(size));
                                    }
                                } else {
                                    return Err(format!("failed to parse size: {}", size_str));
                                }
                            }
                        }
                    }
                }
                _ => {
                    return Err(format!("unknown command {}", cmd).to_owned());
                }
            }
        }

        Ok(root_dir)
    }

    fn new_dir() -> Self {
        Self::Dir(HashMap::new())
    }

    fn entries(&self) -> Option<&HashMap<String, Node>> {
        match self {
            Self::File(_) => None,
            Self::Dir(entries) => Some(entries),
        }
    }

    fn entries_mut(&mut self) -> Option<&mut HashMap<String, Node>> {
        match self {
            Self::File(_) => None,
            Self::Dir(entries) => Some(entries),
        }
    }

    fn find_path_mut(&mut self, path: &[String]) -> Option<&mut Node> {
        if path.is_empty() {
            Some(self)
        } else {
            match self {
                Self::File(_) => None,
                Self::Dir(entries) => entries
                    .get_mut(&path[0])
                    .map(|node| node.find_path_mut(&path[1..]))
                    .unwrap_or(None),
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Dir(entries) => entries.iter().map(|(_, node)| node.size()).sum(),
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Self::Dir(_) => true,
            _ => false,
        }
    }

    fn sum_all_directory_sizes_below(&self, max_size: usize) -> usize {
        match self {
            Self::File(_) => 0,
            Self::Dir(entries) => {
                Some(self.size()).filter(|s| *s <= max_size).unwrap_or(0)
                    + entries
                        .iter()
                        .map(|(_, node)| node.sum_all_directory_sizes_below(max_size))
                        .sum::<usize>()
            }
        }
    }

    fn find_directory_by_size_lowerbound(&self, lowerbound: usize) -> usize {
        match self {
            Self::File(_) => usize::MAX,
            Self::Dir(entries) => *Some(self.size())
                .iter()
                .filter(|s| (**s) >= lowerbound)
                .chain(
                    entries
                        .iter()
                        .map(|(_, node)| node.find_directory_by_size_lowerbound(lowerbound))
                        .collect::<Vec<usize>>()
                        .iter(),
                )
                .min()
                .unwrap_or(&usize::MAX),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fn write_indent(
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> Result<(), std::fmt::Error> {
            for _ in 0..depth {
                write!(f, "  ")?;
            }
            Ok(())
        }

        fn disp(
            f: &mut std::fmt::Formatter<'_>,
            node: &Node,
            name: &str,
            depth: usize,
        ) -> Result<(), std::fmt::Error> {
            match node {
                Node::File(siz) => {
                    write_indent(f, depth)?;
                    write!(f, "{} (size={})\n", name, siz)?;
                }
                Node::Dir(entries) => {
                    write_indent(f, depth)?;
                    write!(f, "{}/", name)?;

                    write!(f, " (size={})", node.size())?;

                    write!(f, "\n")?;

                    for (name, file) in entries {
                        disp(f, file, name, depth + 1)?;
                    }
                }
            };
            Ok(())
        }

        disp(f, self, "", 0)
    }
}

fn main() {
    // let filename = "test.in";
    let filename = "my.in";
    let input = fs::read_to_string(filename).unwrap();
    let lines = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .peekable();

    let root = Node::from_command_lines(&mut lines.clone()).expect("commands must parse correctly");

    println!("Part 1:");
    println!("{}", root.sum_all_directory_sizes_below(100000));

    let total_disk_space = 70000000;
    let space_needed = 30000000;

    let space_to_free = space_needed - (total_disk_space - root.size());

    let space_freed = root.find_directory_by_size_lowerbound(space_to_free);

    println!("Part 2:");
    println!("{}", space_freed);
}
