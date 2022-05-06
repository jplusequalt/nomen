use home;
use std::path::{Path, PathBuf};

pub fn split_arguments(args: &mut String) -> Vec<&str> {
    args.split_whitespace().collect()
}

pub fn strip_home_prefix(p: &mut PathBuf) -> Option<PathBuf> {
    if p.starts_with("~") {
        if p == Path::new("~") {
            return home::home_dir();
        }

        let strip_path = p.strip_prefix("~").unwrap().to_path_buf();

        let mut new_path = home::home_dir().unwrap();
        new_path.push(strip_path);

        return Some(new_path);
    } else if p.starts_with("$HOME") {
        if p == Path::new("$HOME") {
            return home::home_dir();
        }

        let strip_path = p.strip_prefix("$HOME").unwrap().to_path_buf();

        let mut new_path = home::home_dir().unwrap();
        new_path.push(strip_path);

        return Some(new_path);
    }

    Some(p.to_path_buf())
}

pub fn get_alias_file() -> Option<PathBuf> {
    let mut alias_path = match home::home_dir() {
        Some(h) => h,
        None => return None,
    };

    alias_path.push(".bash_aliases");

    if !alias_path.exists() {
        return None;
    }

    Some(alias_path)
}
