// https://adventofcode.com/2022/day/7

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const DIR_SIZE_LIMIT: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const FREE_SPACE_NEEDED: u32 = 30_000_000;

pub fn solve(input: String) -> (String, String) {
    let root = Rc::new(RefCell::new(FileEntry::new_dir()));
    let mut cd = root.clone();

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let cd_name = &line[5..];
            cd_to(&mut cd, &root, cd_name);
        } else if line.starts_with("$ ls") {
            // ignore
        } else if line.starts_with("dir ") {
            let dir_name = &line[4..];
            let new_dir = cd.borrow_mut().new_subdir(dir_name);
            new_dir.borrow_mut().with_parent(cd.clone());
        } else {
            let mut split = line.split(' ');
            let size = split
                .next()
                .expect("no file size")
                .parse()
                .expect("failed to parse file size");
            cd.borrow_mut().file(size);
        }
    }

    let p1 = root.borrow().size_limit_recursive(DIR_SIZE_LIMIT);

    let used_space = root.borrow().size_recursive();
    let free_space = TOTAL_DISK_SPACE - used_space;
    let space_deficit = FREE_SPACE_NEEDED - free_space;

    let dir_sizes = root.borrow().dir_sizes_recursive();
    let dir_size_to_del = *dir_sizes
        .iter()
        .filter(|dir_size| **dir_size >= space_deficit)
        .min()
        .expect("No dir sizes match the criteria");

    (p1.to_string(), dir_size_to_del.to_string())
}

fn cd_to(cd: &mut Rc<RefCell<FileEntry>>, root: &Rc<RefCell<FileEntry>>, cd_name: &str) {
    match cd_name {
        "/" => {
            *cd = root.clone();
        }
        ".." => {
            let new_cd = cd
                .borrow()
                .parent
                .as_ref()
                .expect("Tried to cd to the parent of `/`")
                .clone();
            *cd = new_cd;
        }
        other => {
            let new_cd = cd
                .borrow()
                .subdir(other)
                .expect(&*format!("Subdir {} does not exist", other))
                .clone();
            *cd = new_cd;
        }
    }
}

struct FileEntry {
    size: u32,
    parent: Option<Rc<RefCell<FileEntry>>>,
    sub_dir: HashMap<String, Rc<RefCell<FileEntry>>>,
}

impl FileEntry {
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn new_dir() -> Self {
        FileEntry {
            size: 0,
            parent: None,
            sub_dir: Default::default(),
        }
    }

    pub fn file(&mut self, size: u32) {
        self.size += size;
    }

    pub fn new_subdir(&mut self, name: impl ToString) -> Rc<RefCell<Self>> {
        let child = Self::new_dir().wrap();
        self.sub_dir.insert(name.to_string(), child.clone());
        child
    }

    pub fn with_parent(&mut self, parent: Rc<RefCell<FileEntry>>) {
        self.parent = Some(parent);
    }

    pub fn subdir(&self, name: &str) -> Option<Rc<RefCell<Self>>> {
        self.sub_dir.get(name).map(|d| d.clone())
    }

    pub fn size_recursive(&self) -> u32 {
        return self.size
            + self
                .sub_dir
                .values()
                .map(|subdir| subdir.borrow().size_recursive())
                .sum::<u32>();
    }

    pub fn size_limit_recursive(&self, limit: u32) -> u32 {
        let mut size = self.size_recursive();
        if size > limit {
            size = 0;
        }
        size += self
            .sub_dir
            .values()
            .map(|subdir| subdir.borrow().size_limit_recursive(limit))
            .sum::<u32>();
        return size;
    }

    pub fn dir_sizes_recursive(&self) -> Vec<u32> {
        let mut out = vec![self.size_recursive()];

        for subdir in self.sub_dir.values() {
            for dir_size in subdir.borrow().dir_sizes_recursive() {
                out.push(dir_size)
            }
        }

        out
    }
}
