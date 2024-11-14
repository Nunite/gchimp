use std::{
    fs,
    path::{Path, PathBuf},
};

use super::constants::TEXTURE_PREFIXES;

use crate::err;

pub fn maybe_add_extension_to_string(s: &str, ext: &str) -> String {
    let ext_with_dot = format!(".{}", ext);

    if s.ends_with(&ext_with_dot) {
        s.to_string()
    } else {
        format!("{}.{}", s, ext)
    }
}

pub fn find_files_with_ext_in_folder(path: &Path, ext: &str) -> std::io::Result<Vec<PathBuf>> {
    let rd = fs::read_dir(path)?;
    let paths = rd.filter_map(|path| path.ok()).map(|path| path.path());
    let ext_paths = paths
        .filter(|path| path.extension().is_some() && path.extension().unwrap() == ext)
        .collect();

    Ok(ext_paths)
}

pub fn relative_to_less_relative(root: &Path, relative: &Path) -> PathBuf {
    root.join(relative)
}

// i use linux to do things
pub fn fix_backslash(i: &str) -> String {
    i.replace("\\", "/")
}

pub fn remove_texture_prefix(i: &str) -> String {
    TEXTURE_PREFIXES
        .iter()
        .fold(i.to_owned(), |acc, e| acc.replace(e, ""))
}

pub fn parse_triplet(i: &str) -> eyre::Result<[f64; 3]> {
    let res = i
        .split_ascii_whitespace()
        .filter_map(|i| i.parse::<f64>().ok())
        .collect::<Vec<f64>>();

    if res.len() < 3 {
        return err!("Cannot parse triplet of number: {}", i);
    }

    Ok([res[0], res[1], res[2]])
}

#[macro_export]
macro_rules! err {
    ($e: ident) => {{
        use eyre::eyre;

        Err(eyre!($e))
    }};

    ($format_string: literal) => {{
        use eyre::eyre;

        Err(eyre!($format_string))
    }};

    ($($arg:tt)*) => {{
        use eyre::eyre;

        Err(eyre!($($arg)*))
    }};
}

#[macro_export]
macro_rules! rand_int_range {
    ($x1:expr,$x2:expr) => {{
        (rand::random::<f32>() * ($x2 - $x1) as f32 + $x1 as f32).round() as u32
    }};
}
