use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags{
    pub n: bool,
    pub l: bool,
    pub i: bool,
    pub v: bool,
    pub x: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut parameter = Flags::default();
        for flag in flags{
            match flag.trim_start_matches("-"){
                "n"=> parameter.n = true,
                "l"=> parameter.l = true,
                "i"=> parameter.i = true,
                "v"=> parameter.v = true,
                "x"=> parameter.x = true,
                _ => (),
            };
        }
        parameter
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut pos = Vec::new();
    let multiple_files_flag = files.len() > 1;

    for filename in files.iter(){
        let contents = fs::read_to_string(filename)?;
        if  (contents.contains(pattern) && !flags.i) ||
            (contents.to_lowercase().contains(&pattern.to_lowercase()) && flags.i) ||
            (!contents.contains(pattern) && flags.v)
        {
            for (line_index, line) in contents.lines().enumerate(){
                if  (!line.contains(pattern) && flags.v) ||
                    (line == pattern && flags.x) ||
                    (line.to_lowercase().contains(&pattern.to_lowercase()) && flags.i) ||
                    (line.contains(pattern) && !flags.x)
                {
                    if (line != pattern && flags.v && flags.x) || (!line.contains(pattern) && flags.v){
                        if multiple_files_flag{
                            pos.push(format!("{}:{}",filename, line.to_string()));
                            continue;
                        }else{
                            pos.push(line.to_string());
                            continue;
                        }
                    }
                    if flags.l{
                        if !pos.contains(&filename.to_string()){
                            pos.push(filename.to_string());
                        }
                        continue;

                    }

                    if multiple_files_flag && (!flags.n && !flags.l && !flags.v){
                        pos.push(format!("{}:{}",filename, line.to_string()));
                        continue;
                    }
                    if multiple_files_flag && flags.n{
                        pos.push(format!("{}:{}:{}",filename, line_index+1, line.to_string()));
                        continue;
                    }


                    if flags.n{
                        pos.push(format!("{}:{}",line_index+1, line.to_string()));
                        continue;
                    }

                    if !multiple_files_flag && !flags.v{
                        pos.push(line.to_string());
                    }
                }
            }
        }
    }
    Ok(pos)
}

pub fn main(){

}
