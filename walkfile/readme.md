# walkfile

# Struct walkfile::WalkFileEntry
```rust
pub struct WalkFileEntry {
    pub root: PathBuf,
    pub child_dirs: Vec<String>,
    pub child_files: Vec<String>,
}
```
## WalfFileENtry
* root： 根目录
* child_dirs: root下所有的子文件夹名称
* child_files: root下所有的文件
# Function walkfile::walkCopy

`pub fn walk(path: &PathBuf) -> Result<Vec<WalkFileEntry>>` 

遍历指定根目录的入口函数
```rust
use std::{path::PathBuf, str::FromStr};
use crate::walk;
use anyhow::{Ok, Result}; 
fn test_walk(){
let res = walk(&PathBuf::from_str("./").unwrap());
let res = match res {
    Err(e) => panic!("{}", e),
    Ok(res) => res
};
print!("{}", res[1]);
} 
```

# 引入
cargo.toml
```toml
[dependencies]
walkfile = "*"
```
# 使用
```rust
fn test_ref_tuple(){
    let res = walk(&PathBuf::from_str("./").unwrap());
    let res = match res {
        Err(e) => panic!("{}", e),
        Ok(res) => res
    };
    let (root, child_dirs, child_files) = res[0].as_tuple();
    println!("{:?}, {:?}, {:?}", root, child_dirs, child_files);
}
```
### 输出
```json
[
    WalkFileEntry {
        root: "./src",
        child_dirs: [],
        child_files: [
            "lib.rs",
        ],
    },
    WalkFileEntry {
        root: "./",
        child_dirs: [
            "src",
        ],
        child_files: [
            "Cargo.toml",
            "readme.md",
        ],
    },
]
```