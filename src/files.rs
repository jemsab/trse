pub struct File {
    pub name: String,
    pub sub_elements: Vec<File>,
    pub depth: u32,
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
    fn pp_file_aux(file: &File) {
        let files = &file.sub_elements;
        for file in files {
            pp_file_aux(&file);
        }
        println!(
            "{}-- {}",
            "   |".repeat(file.depth.try_into().unwrap()),
            file.name.as_str()
        );
    }
    println!("{}", file.name.as_str());
    pp_file_aux(file);
    println!();
}

pub fn recurse_dirs(start: &str, max_depth: u32) -> File {
    fn recurse_dirs_aux(start: &str, depth: u32, max_depth: u32) -> File {
    let mut root = File {
        name: String::from(start),
        sub_elements: Vec::new(),
        depth,
    };

    if let Ok(dirs) = std::fs::read_dir(start) {
        for dir in dirs {
            if let Ok(dir) = dir {
                let file_name = dir.file_name();
                if let Some(file_name) = file_name.to_str() {
                    if let Ok(file_type) = dir.file_type() {
                        let sub_elements = if file_type.is_dir() && depth < max_depth {
                            println!("{} is dir {}", file_name, file_type.is_dir());
                            recurse_dirs_aux(dir.path().to_str().unwrap(), depth + 1, max_depth)
                                .sub_elements
                        } else {
                            Vec::new()
                        };
                        let file = File {
                            name: String::from(file_name),
                            sub_elements,
                            depth,
                        };
                        root.sub_elements.push(file);
                    }
                }
            }
        }
    }
    root
    }
    recurse_dirs_aux(start, 0, max_depth)
}
