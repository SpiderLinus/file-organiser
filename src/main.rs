use std::env;
use std::fs;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: file_organizer <path>");
        return;

    }

    let path= &args[1];
    println!("Organizing: {}", path);

    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let path_buf = entry.path();

        if path_buf.is_dir() {
            continue;
        }

        let extension = match path_buf.extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => String::from("no_extension"),
        };


        let target_dir = Path::new(path).join(&extension);

        if !target_dir.exists() {
            fs::create_dir(&target_dir).unwrap();
        }

        let target_file = target_dir.join(&file_name);
        fs::rename(entry.path(), &target_file).unwrap();

        println!("Moved {:?} -> {}", file_name, extension);
    }

    println!("Folder Organized!, Thanks!");
}
