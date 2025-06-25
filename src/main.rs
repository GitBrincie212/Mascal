use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs};
use mascal::language_pipeline;

fn merge_paths(input_path: &Path, lang_dir: &Path) -> Option<PathBuf> {
    if input_path.is_absolute() {
        return Some(input_path.to_path_buf());
    }
    let absolute_path = lang_dir.join(input_path).canonicalize();
    if absolute_path.is_err() {
        println!(
            "\x1b[1;31mUnexpected error when trying to merge the relative path of the provided file \x1b[0m"
        );
        return None;
    }
    Some(absolute_path.unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lang_path = env::current_exe();
    if args.len() <= 1 {
        println!(
            "\x1b[1;31mPlease provide at least one argument (that being the file to run)\x1b[0m"
        );
        exit(64)
    }
    if lang_path.is_err() {
        println!("\x1b[1;31mUnexpected error when trying to get the executable's path\x1b[0m");
        return;
    }
    let extracted_lang_path = lang_path.unwrap();
    if extracted_lang_path.parent().is_none() {
        println!(
            "\x1b[1;31mUnexpected error when trying to get the executable file's directory \x1b[0m"
        );
        return;
    }
    let input_path = Path::new(&args[1]);
    let lang_dir = extracted_lang_path.parent().unwrap();
    let path: Option<PathBuf> = merge_paths(input_path, lang_dir);
    if path.is_none() {
        return;
    }
    let path = path.unwrap();
    let does_exist = fs::exists(&path);
    if does_exist.is_err() {
        println!("\x1b[1;31mUnexpected Error when determing if the provided file exists\x1b[0m");
        return;
    }
    if !does_exist.unwrap() {
        println!("\x1b[1;31mProvided file doesn't exist\x1b[0m");
        return;
    }
    let extension = &path.extension().and_then(OsStr::to_str);
    if extension.is_none() {
        println!(
            "\x1b[1;31mUnexpected Error when determing the extension of the provided file\x1b[0m"
        );
        return;
    }
    let extension = extension.unwrap();
    if !extension.eq("mascal") {
        println!("\x1b[1;31mProvided file isn't a .mascal extension\x1b[0m");
        return;
    }
    let contents = fs::read_to_string(&path);
    if contents.is_err() {
        println!("{:?}", contents.err());
        println!("\x1b[1;31mUnexpected Error when reading the provided mascal file\x1b[0m");
        return;
    }
    let contents = contents.unwrap();
    language_pipeline::trigger_pipeline(contents);
}
