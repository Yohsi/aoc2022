use colored::*;
use std::{fs, vec, cell::RefCell, rc::Rc};

const DAY: u32 = 7;

fn main() {
    let sample = fs::read_to_string(format!("samples/day{:02}", DAY)).unwrap_or_default();
    let input = fs::read_to_string(format!("inputs/day{:02}", DAY)).unwrap_or_default();

    println!("{}{}", "Part 1 sample: ".bold().yellow(), format!("{}", part1(&sample)).bright_white());
    println!("{}{}", "Part 1 input : ".bold().blue(), format!("{}", part1(&input)).bright_white());
    println!("{}{}", "Part 2 sample: ".bold().yellow(), format!("{}", part2(&sample)).bright_white());
    println!("{}{}", "Part 2 input : ".bold().blue(), format!("{}", part2(&input)).bright_white());
}

struct File {
    name: String,
    size: u64,
}

struct Folder {
    name: String,
    parent: Option<Rc<Folder>>,
    subfolders: RefCell<Vec<Rc<Folder>>>,
    files: RefCell<Vec<File>>,
}

impl Folder {
    fn total_size(&self) -> u64 {
        return self.files.borrow().iter().map(|f| f.size).sum::<u64>()
            + self.subfolders.borrow().iter().map(|f| f.total_size()).sum::<u64>();
    }

    fn folder_from_rel_path(&self, path: &str) -> Rc<Folder> {
        match path {
            ".." => self.parent.as_ref().unwrap().clone(),
            folder_name => {
                self.subfolders.borrow()
                    .iter()
                    .find(|f| f.name == folder_name)
                    .unwrap()
                    .clone()
            }
        }
    }

    fn sum_folders_less_than(&self, size: u64) -> u64 {
        let mut total_size = self.total_size();
        if total_size > size {
            total_size = 0;
        }
        self.subfolders.borrow().iter().fold(total_size, |acc, f| acc + f.sum_folders_less_than(size))
    }

    fn find_closest_from(&self, target: u64) -> u64 {
        let mut current_best = u64::MAX;
        let mut current_best_size = 0;

        let size = self.total_size();
        if size > target && size - target < current_best {
            current_best = size - target;
            current_best_size = size;
        }

        for subfolder in self.subfolders.borrow().iter() {
            let size = subfolder.find_closest_from(target);
            if size > target && size - target < current_best {
                current_best = size - target;
                current_best_size = size;
            }
        }
        current_best_size
    }
}

fn create_filesystem(input: &str) -> Rc<Folder> {
    let root = Rc::new(Folder {
        name: String::new(),
        parent: None,
        files: RefCell::new(vec![]),
        subfolders: RefCell::new(vec![]),
    });

    let mut current_folder = root.clone();

    for line in input.lines() {
        if line.starts_with("$ ls") {
            //
        } else if line.starts_with("$ cd /") {
            current_folder = root.clone();
        } else if line.starts_with("$ cd ") {
            let rel_path = line.strip_prefix("$ cd ").unwrap();
            current_folder = current_folder.folder_from_rel_path(rel_path);
        } else if line.starts_with("dir") {
            let folder_name = line.strip_prefix("dir ").unwrap();
            current_folder.subfolders.borrow_mut().push(Rc::new(Folder {
                name: folder_name.to_owned(),
                parent: Some(current_folder.clone()),
                files: RefCell::new(vec![]),
                subfolders: RefCell::new(vec![]),
            }));
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            current_folder.files.borrow_mut().push(File { name: name.to_owned(), size: size.parse().unwrap() });
        }
    }
    root
}

fn part1(input: &str) -> u64 {
    let root = create_filesystem(input);
    root.sum_folders_less_than(100000)
}

fn part2(input: &str) -> u64 {
    const TOTAL_SIZE: u64 = 70000000;
    const NEEDED_SPACE: u64 = 30000000;
    let root = create_filesystem(input);

    let free_space = TOTAL_SIZE - root.total_size();
    root.find_closest_from(NEEDED_SPACE - free_space)
}
