pub mod args;
pub use args::Args;

use std::error::Error;
use std::io::{self, BufRead};
use colored::Colorize;


fn get_line(lines: &Vec<String>, pos: usize) -> Result<&String, &'static str> {
    let l = &lines.get(pos);
    if l.is_none() {
        return Err("Element doesn't exists");
    }
    Ok(l.unwrap())
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|x| String::from(x.expect("Map error")))
        .collect::<Vec<String>>();

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
            let pos_line = get_line(&lines, read_from)?;
            if pos_line.find('{').is_some() {
                nesting -= 1;
            }
            if pos_line.find('}').is_some() {
                nesting += 1;
            }
        }

        let mut read_to = occ.clone();
        nesting = args.nested_recursion;
        while nesting != 0 {
            let pos_line = get_line(&lines, read_to)?;
            if pos_line.find('{').is_some() {
                nesting += 1;
            }
            if pos_line.find('}').is_some() {
                nesting -= 1;
            }
            if nesting != 0 {
                read_to += 1;
            }
        }

        for i in read_from..=read_to {
            if let Some(pos) = lines[i].find(&args.find_word) {
                let l = get_line(&lines, i)?;
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
    Ok(())
}

