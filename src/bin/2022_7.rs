use std::fs::read_to_string;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

fn test_input() -> String {
    "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .to_string()
}
    #[test]
    fn part1_test() {
        let s = test_input();
        let r = part1(&s);
        assert_eq!(r, 95437);
    }
    #[test]
    fn part2_test() {
        let s = test_input();
        let r = part2(&s);
        assert_eq!(r, 24933642);
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
    children: Vec<String>,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        File {
            name,
            size,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Files { 
    files: HashMap<String, Box<File>>,
}

impl Files {
    fn new() -> Self {
        let mut files = HashMap::new();
        let root = File::new(String::from("/"), 0);
        files.insert(root.name.clone(), Box::new(root));
        Files { files }
    }
    fn insert(&mut self, file: File) {
        self.files.insert(file.name.clone(), Box::new(file));
    }
    fn get(& self, name: &str) -> Option<&File> {
        self.files.get(name).map(|f| f.as_ref())
    }
    fn add_child(&mut self, parent: &str, child: &str) {
        let parent = self.files.get_mut(parent).unwrap();
        parent.children.push(child.to_string());
    }
}

fn get_files(s: &String) -> Files{
    let mut files = Files::new();

    let mut stack: Vec<String> = Vec::new();
    stack.push("/".to_string());

    let mut lines = s.lines();
    lines.next();
    for line in lines {
        //split line on spaces and return a slice
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        match tokens[..] {
            ["$","cd", dir] => {
                match dir {
                    "/" => {
                        stack.clear();
                        stack.push("/".to_string());
                    },
                    ".." => {
                        stack.pop();
                    },
                    dir_name => {
                        let dir_name: String = stack.last().unwrap().to_string() + dir_name;
                        stack.push(dir_name);
                    }
                }
            }
            ["$","ls"] => {
                continue;
            }
            ["dir", dir_name] => {
                let dir_name: String = stack.last().unwrap().to_string() + dir_name;
                match files.get(&dir_name) {
                    Some(_) => {
                        continue;
                    },
                    None => {
                        let dir = File::new(dir_name.to_string(), 0);
                        files.insert(dir);
                        files.add_child(stack.last().unwrap(), &dir_name);
                    }
                }
            }
            [size, file_name] => {
                let file_name: String = stack.last().unwrap().to_string() + file_name;
                let current_dir_name = stack.last().unwrap();
                let file = File::new(file_name.to_string(), size.parse().unwrap());
                files.insert(file);
                files.add_child(current_dir_name, &file_name);
            }
            _ => {
                panic!("invalid line: {}", line);
            }
        }
    }
    files
}


fn get_total_size(files: &Files) -> HashMap<String, usize> {
    //iterative postorder traversal
    let mut stack = Vec::new();
    let mut post_order: Vec<String> = Vec::new();
    let mut total_size = HashMap::new();
    stack.push("/");
    while !stack.is_empty() {
        let dir_name = stack.pop().unwrap();
        let dir = files.get(dir_name).unwrap();
        post_order.push(dir_name.to_string());
        for child in dir.children.iter() {
            let child_file = files.get(&child).unwrap();
            if child_file.size == 0 {
                stack.push(child);
            }
        }
    }
    post_order.reverse();
    for dir_name in post_order.iter() {
        let dir = files.get(dir_name).unwrap();
        let sum: usize = dir.children
            .iter()
            .map(|child| {
                 match files.get(child).unwrap().size {
                     0 => total_size[child],
                     size => size,
                 }
                
            }
           ) 
            .sum();
        total_size.insert(dir.name.clone(), sum.clone());
    }
    total_size
}

const TOTAL_DISK_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

fn part1 (s: &String) -> usize {
    let files = get_files(s);
    let total_size: HashMap<String, usize> = get_total_size(&files);
    //total size is the sum of the size of all files with size <= 100000
    total_size
        .iter()
        .map(|(_, size)| if *size <= 100000 { *size } else { 0 })
        .sum()
}

fn part2 (s: &String) -> usize {
    let files = get_files(s);
    let total_size: HashMap<String, usize> = get_total_size(&files);
    let space_left = TOTAL_DISK_SPACE - total_size["/"];
    let space_missing = NEEDED_SPACE - space_left;
    let possible_deletes = total_size.iter().filter(|(_, size)| *size >= &space_missing);
    *possible_deletes.map(|(_, size)| size).min().unwrap()
}

fn main () {
    let input = read_to_string("inputs/2022_7.txt").expect("Could not read file");
    let part1 = part1(&input);
    println!("part1: {}", part1);
    let part2 = part2(&input);
    println!("part2: {}", part2);
}

