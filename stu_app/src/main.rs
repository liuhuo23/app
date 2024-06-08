use std::{path::PathBuf, str::FromStr};

fn main() {
    println!("Hello, world!");
    let res = walkfile::walk(&PathBuf::from_str("./").unwrap()).unwrap();
    for dir in res{
        let (root, child_dirs, child_files) = dir.as_tuple_ref();
        println!("{}, {:?}, {:?}", root.display(), child_dirs, child_files);
    }
}
