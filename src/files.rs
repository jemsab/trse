pub struct File {
    pub name: String,
    pub sub_elements: Vec<File>,
    pub depth: i32,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - [\n", self.name,)?;
        for el in &self.sub_elements {
            write!(f, "{}\n", el.name)?;
        }
        write!(f, "]")
    }
}

pub fn pp_file(file: &File) {
    fn pp_file_aux(file: &File, indent: usize) {
        let files = &file.sub_elements;
        for file in files {
            pp_file_aux(&file, indent + 1);
        }
        println!("{}-- {}", "   |".repeat(indent), file.name.as_str());
    }
    println!("{}", file.name.as_str());
    pp_file_aux(file, 0);
    println!();
}

pub fn recurse_dirs(start: &str, depth: i32) -> File {
    let mut root = File {
        name: String::from(start),
        sub_elements: Vec::new(),
        depth,
    };

    if let Ok(dirs) = std::fs::read_dir(start) {
        for dir in dirs {
            if let Ok(dir) = dir {
                if let Ok(_) = dir.file_type() {
                    let file_name = dir.file_name();
                    let file_name = file_name.to_str().unwrap();
                    if let Ok(file_type) = dir.file_type() {
                        let sub_elements = if file_type.is_dir() {
                            recurse_dirs(dir.path().to_str().unwrap(), depth + 1).sub_elements
                        } else {
                            Vec::new()
                        };
                        let file = File {
                            name: String::from(file_name),
                            sub_elements,
                            depth
                        };
                        root.sub_elements.push(file);
                    }
                }
            }
        }
    }
    root
}
