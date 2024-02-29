use minigrep::Config;
use std::env;//传入命令行参数
use std::process;

fn main() {
    //直接将env::args传入，因为它本身就有迭代器
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("出错了，问题所在：{}", err);
        //只显示上面的信息后就退出程序
        process::exit(1);
    });

    //用变量存储文件中的数据
    if let Err(err) = minigrep::run(config) {
        eprintln!("应用错误：{}",err);
        process::exit(1);
    }
}   
