use std::env;
use std::fs;

/*
goal is to make a ls-like program that can be linked in my windows cmd shell
so that I stop getting errors when I type ls without thinking.

call pattern for ls is:
ls [OPTIONS] [FILE]

if called without args just print the current dir

*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();

    let mut target_dir = String::from(".");

    if arg_len == 2 { // then we change the print or list a different dir
        target_dir = args[1].clone();
    } else if arg_len > 2{
        // some sort of clever arg parsing or something
        println!("was called with args:{:?} and has len {}", args, arg_len);
    }

    if let Ok(items) = fs::read_dir(target_dir) {
        for item in items {
            if let Ok(item) = item {
                let os_path = item.file_name();
                let name = os_path.to_string_lossy();
                println!("{}", name);
            }
        }
    }
}
