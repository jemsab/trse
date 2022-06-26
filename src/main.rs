
mod args;
mod files;

fn main() {

    let root = std::env::args().nth(1);
    let root = match root {
        Some(r)  => r,
        None => String::from(".")
    };
    let files = files::recurse_dirs(root.as_str());
    files::pp_file(&files);

}
