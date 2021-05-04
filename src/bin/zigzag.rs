extern crate color_eyre;

use color_eyre::{Result};
use std::io::stdin;

fn main() -> Result<()> {
    println!(
        r#"
         ____  ____   ___    ____    __     ___ 
        (_   )(_  _) / __)  (_   )  /__\   / __)
         / /_  _)(_ ( (_-.   / /_  /(__)\ ( (_-.
        (____)(____) \___/  (____)(__)(__) \___/
             ____  ____  ____  _  _  ____ 
            (  _ \(  _ \(_  _)( \( )(_  _)
             )___/ )   / _)(_  )  (   )(  
            (__)  (_)\_)(____)(_)\_) (__) 
        
        "#
    );
    let mut sentence_string = String::new();
    println!("Enter some text:");
    stdin().read_line(&mut sentence_string)?;
    let sentence: Vec<char> = sentence_string.chars().collect();

    let mut kstring = String::new();
    println!("Enter zigzag height (>0):");
    stdin().read_line(&mut kstring)?;
    let k = kstring.replace("\n","").parse::<usize>()?;

    // Inputs: line, k

    let get_spaces =  |row: usize, desc: bool| -> usize {
        let mut spaces = (k - 1) * 2 - 1; // initialize with maximum possible spaces
        
        if desc {
            spaces -= row * 2;
        } else {
            spaces -= (k - 1 - row) * 2;
        }

        spaces
    };

    let is_descending = |curr_idx| curr_idx % (2 * (k - 1)) < k - 1;

    let n = sentence.len();

    for row in 0usize..k {
        let mut line = vec![' '; n];
        let mut i: usize = row;
        while i < n {
            line[i] = sentence[i];
            let is_going_down = is_descending(i);
            let spaces = get_spaces(row, is_going_down);
            i += spaces + 1;
        }

        println!("{}", &line.iter().clone().collect::<String>());
    }

    Ok(())
}

