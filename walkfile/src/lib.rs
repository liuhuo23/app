use std::{fs, path::PathBuf, vec};
use anyhow::{Ok, Result};

/// # WalfFileENtry
/// * root： 根目录
/// * child_dirs: root下所有的子文件夹名称
/// * child_files: root下所有的文件
#[derive(Debug, Clone)]
pub struct WalkFileEntry{
    pub root: PathBuf,
    pub child_dirs: Vec<String>,
    pub child_files: Vec<String>,
}

impl WalkFileEntry {
    /// 将属性以引用的方式构造成元组
    /// ```rust
    /// use std::{path::PathBuf, str::FromStr};
    /// use crate::walk;
    /// use anyhow::{Ok, Result};
    /// fn test_ref_tuple(){
    ///     let res = walk(&PathBuf::from_str("./").unwrap());
    ///     let res = match res {
    ///        Err(e) => panic!("{}", e),
    ///         Ok(res) => res
    ///     };
    ///     let (root, child_dirs, child_files) = res[0].as_tuple_ref();
    ///     println!("{:?}, {:?}, {:?}", root, child_dirs, child_files);
    /// }
    /// ```
    pub fn as_tuple_ref(&self)->(&PathBuf, &Vec::<String>, &Vec::<String>){
        (&self.root, &self.child_dirs, &self.child_files)
    }

    /// 将属性以克隆的方式构造成元组
    /// ```rust
    /// use std::{path::PathBuf, str::FromStr};
    /// use crate::walk;
    /// use anyhow::{Ok, Result};
    /// fn test_ref_tuple(){
    ///     let res = walk(&PathBuf::from_str("./").unwrap());
    ///     let res = match res {
    ///        Err(e) => panic!("{}", e),
    ///         Ok(res) => res
    ///     };
    ///     let (root, child_dirs, child_files) = res[0].as_tuple();
    ///     println!("{:?}, {:?}, {:?}", root, child_dirs, child_files);
    /// }
    /// ``` 
    pub fn as_tuple(&self)->(PathBuf, Vec::<String>, Vec::<String>){
        (self.root.clone(), self.child_dirs.clone(), self.child_files.clone())
    }
}


/// 为WalkFileEntry 实现disply
impl std::fmt::Display for WalkFileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "root: {}, child_dirs: {:?}, child_files:{:?}", self.root.display(), self.child_dirs, self.child_files)
    }
}

/// 遍历指定根目录的入口函数
/// ```rust
///use std::{path::PathBuf, str::FromStr};
/// use crate::walk;
/// use anyhow::{Ok, Result}; 
/// fn test_walk(){
/// let res = walk(&PathBuf::from_str("./").unwrap());
/// let res = match res {
///     Err(e) => panic!("{}", e),
///     Ok(res) => res
/// };
/// print!("{}", res[1]);
/// } 
/// ```
pub fn walk(path: &PathBuf)->Result<Vec<WalkFileEntry>>{
    let mut res:Vec<WalkFileEntry> = Vec::new();
    walk_dir(path, &mut res)?;
    Ok(res)
}

/// 递归的获取子文件夹， 子文件
fn walk_dir(path: &PathBuf, res: &mut Vec<WalkFileEntry>)->Result<()>{
    let mut walk = WalkFileEntry{
        root: path.clone(),
        child_dirs: vec![],
        child_files: vec![],
    };
    for entry in fs::read_dir(path)?{
        let entry = entry?;
        if entry.path().is_dir(){
            if let Some(p) = entry.path().file_name(){
                if let Some(p_str) = p.to_str(){
                    walk.child_dirs.push(p_str.to_string());
                }
            }
            walk_dir(&entry.path(), res)?;
        }else if entry.path().is_file() {
            if let Some(p) = entry.path().file_name(){
                if let Some(p_file) = p.to_str(){
                    walk.child_files.push(p_file.to_string());
                }
            }
        }
    }
    res.push(walk);
    Ok(())
}

#[cfg(test)]
mod test{
    use std::{path::PathBuf, str::FromStr};

    use crate::walk;

    #[test]
    fn test_walk(){
        let res = walk(&PathBuf::from_str("./").unwrap());
        let res = match res {
            Err(e) => panic!("{}", e),
            Ok(res) => res
        };
        print!("{:#?}", res);
    }

    #[test]
    fn test_ref_tuple(){
        let res = walk(&PathBuf::from_str("./").unwrap());
        let res = match res {
            Err(e) => panic!("{}", e),
            Ok(res) => res
        };
        let (root, child_dirs, child_files) = res[0].as_tuple();
        println!("{:?}, {:?}, {:?}", root, child_dirs, child_files);
    }
}