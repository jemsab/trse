
mod args;
mod files;

fn main() {

    let args = args::parse_args();

    let files = files::recurse_dirs(args.directory.as_str(), args.depth);
    files::pp_file(&files);
}
