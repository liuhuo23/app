use std::{collections::HashMap, env, ffi::OsStr, fs::{self, File}, io::Read, path::PathBuf, str::FromStr};
use anyhow::{Ok, Result};

const FILE_NAME:[&str;4] = ["py", "rs", "go","java"];
fn main()->Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        panic!("必须指定参数")
    }
    let path = PathBuf::from_str(args[1].as_str())?;
    if !path.exists(){
        panic!("指定的文件夹不存在")
    }
    let mut state:HashMap<String, i32> = HashMap::new();
    println!("开始统计指定目录下不同语言的代码行数");
    statistics_line(&mut state, path)?;
    let values = state.values().map(|v| *v).into_iter().collect::<Vec::<i32>>().into_iter().fold(0, |acc, x| acc + x);
    if values == 0 {
        println!("未统计到代码");
        return Ok(());
    }
    for (k, v) in state.iter(){
        println!("{}有{}行占比 {}", k, v, format!("{:.2}%", (*v as f64) / (values as f64) * 100.0));
    }
    Ok(())
}

fn statistics_line(state: &mut HashMap<String, i32>, path: PathBuf)->Result<()>{
    for entry in fs::read_dir(path)?{
        let entry = entry?;
        if is_hidden(entry.path()){
            continue;
        }
        if entry.file_type()?.is_dir(){
            statistics_line(state, entry.path())?;
        }
        if entry.file_type()?.is_file(){
            let path = entry.path();
            let extension = match path.extension() {
                None=> continue,
                Some(a) => a,
            };
            FILE_NAME.iter().find(|file_name| OsStr::new(file_name) == extension).map(|file_name|->Result<()>{
                write_lines(state, path, &file_name.to_string())?;
                Ok(())
            });
        } 
    }
    Ok(())
}

fn write_lines(state: &mut HashMap<String, i32>, path: PathBuf, key: &String)-> Result<()>{
    let mut f = File::open(path)?;
    let mut v: i32 = 0;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    for line in content.lines(){
        if line.len() == 0{
            continue;
        }
        v += 1
    }
    if state.contains_key(key){
        let value = state.get(key);
        match value {
            Some(res_v) => state.insert(key.clone(), res_v + v),
            None => None,
        };
    }else{
        state.insert(key.clone(), v);
    }
    Ok(())
}

fn is_hidden(path: PathBuf)->bool{
    // 以.开头的文件为隐藏文件 以.开头的文件为隐藏文件
    match path.file_name(){
        None => false,
        Some(p) => {
            let temp = p.to_str().unwrap_or_default();
            let temp = String::from(temp);
            if temp.starts_with("."){
                return true
            }
            return false
        }
    }
    
}