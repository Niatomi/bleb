use std::env;

pub struct Args {
    pub find_word: String,
    pub nested_recursion: usize
}

impl Args {
    pub fn new() -> Option<Args> {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            return None;
        }
        let mut nested_recursion: usize = 1;
        let mut find_word: Option<String>= None;
        for arg in args.iter() {
            if args.starts_with(&[String::from("--help")]) {
                if let Ok(val) = arg[1..].parse::<usize>() {
                    nested_recursion = val;
                }
            } else if arg.starts_with('-') {
                return None;
            } else {
                find_word = Some(arg.to_string());
            }
        }
        if let None = find_word {
            return None;
        }
        Some(
            Args { 
                find_word: find_word.expect("Checked"), 
                nested_recursion 
            }
        )
    }

    pub fn help() {
        println!("Usage: [finding_word] [-nesting (int)]");
    }
}
