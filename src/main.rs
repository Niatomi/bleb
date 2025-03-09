use std::env;
use std::process::exit;
use std::io::{self, BufRead};
use colored::Colorize;


fn parse_io(args: Args) {
    let lines = io::stdin().lock().lines().map(|x| String::from(x.expect("Always exists"))).collect::<Vec<String>>();
    let mut occurances: Vec<usize> = Vec::new();
    for (idx, content) in lines.iter().enumerate() {
        if content.find(&args.find_word).is_some() {
            occurances.push(idx);
        }
    }
    
    for occ in occurances.iter() {
        let mut read_from = occ.clone();
        let mut nesting = args.nested_recursion;
        while nesting != 0 {
            read_from -= 1;
            if lines[read_from].find('{').is_some() {
                nesting -= 1;
            }
            if lines[read_from].find('}').is_some() {
                nesting += 1;
            }
        }

        let mut read_to = occ.clone();
        nesting = args.nested_recursion;
        while nesting != 0 {
            if lines[read_to].find('{').is_some() {
                nesting += 1;
            }
            if lines[read_to].find('}').is_some() {
                nesting -= 1;
            }
            if nesting != 0 {
                read_to += 1;
            }
        }

        for i in read_from..=read_to {
            if let Some(pos) = lines[i].find(&args.find_word) {
                let l = &lines[i];
                println!(
                    "{}: {}{}{}", 
                    (i + 1).to_string().green(),
                    &l[..pos], 
                    &l[pos..pos + args.find_word.len()].green(), &l[pos + args.find_word.len()..]
                );
            } else {
                println!("{}: {}", i+1, &lines[i]);
            }
        }
    }

    // match parsing_type {
    //     ParsingType::Curly => handle_curled(args.nested_recursion, cuts, occurs),
    //     ParsingType::Headed => handle_headed(cuts, occurs),
    //     ParsingType::Uknown => handle_unknown(occurs)
    // }
}

struct Args {
    find_word: String,
    nested_recursion: usize
}

fn parse_args() -> Option<Args> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return None;
    }
    let mut nested_recursion: usize = 1;
    let mut find_word: Option<String>= None;
    for arg in args.iter() {
        if arg.starts_with('-') {
            if let Ok(val) = arg[1..].parse::<usize>() {
                nested_recursion = val;
            }
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
            nested_recursion })
}

fn main() {
    let args = parse_args();
    if let None = args {
        exit(0);
    }
    let args = args.expect("Checked");
    parse_io(args);
}
