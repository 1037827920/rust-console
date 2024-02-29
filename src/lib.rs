use core::arch;
use std::error::Error;
use std::fs;
use std::env;//环境变量

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        seach_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for v in results {
        println!("{}", v);
    }

    Ok(())
}

pub struct Config {
    //里面的结构体元素也要pub
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //Config构造函数\
    //这里也要pub
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("需要两个参数");
        }
        args.next();//第一个参数是exe文件名，不需要
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("需要一个字符串"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("需要一个文件名"),
        };

        //只需要检查这个环境变量是否出现，出现为true，不出现为false
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config { query, filename ,case_sensitive});
    }
}

//返回值是从contents取得的，所以要有同样的生命周期
pub fn seach_case_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    //使用迭代器改进
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    //使用迭代器改进
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],seach_case_sensitive(query, contents));
        
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],search_case_insensitive(query, contents));
    }
}