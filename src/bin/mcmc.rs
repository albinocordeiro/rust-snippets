// Simplest Ever Markov Chain Monte Carlo (MCMC) simulation 
extern crate rand;
extern crate color_eyre;
#[macro_use]
extern crate std;
extern crate textplots;

use std::collections::{HashMap};
use rand::prelude::*;
use color_eyre::{eyre, Result};
use textplots::{Chart, Plot, Shape};

struct TransitionProb {
    origin: char,
    dest: char,
    jump_prob: f64
}

fn next_state(start: char, prob_map: &HashMap<char, HashMap<char, f64>>) -> Result<char> {
    let mut r: f64 = random();
    let mut end: char = ' ';
    if let Some(dest_prob_map) = prob_map.get(&start) {
        for (dest, prob) in dest_prob_map.iter() {
            r -= *prob;
            if r <= 0f64 {
                end = *dest;
                break;
            }
        }
    } else {
        return Err(eyre::eyre!("Invalid state provided to transition prob map"));
    }
    Ok(end)
}

fn monte_carlo_run(start: char, transition_probabilities: &Vec<TransitionProb>, walk_distance: i64) -> Result<(Vec<(f32,f32)>, HashMap<char, i64>)> {
    let mut hist = HashMap::new();
    let mut count = 0i64;
    let mut curr_state = start;
    let prob_map = transition_prob_list_to_map(transition_probabilities)?;
    let mut random_walk: Vec<(f32, f32)> = Vec::new();

    while count < walk_distance {
        let state_count = hist.entry(curr_state).or_insert(0);
        *state_count += 1;
        random_walk.push((count as f32, (curr_state as i32 + 1 - 'a' as i32) as f32));
        curr_state = next_state(curr_state, &prob_map)?;    
        count += 1;
    }

    Ok((random_walk, hist))
}

fn visits_histogram_to_string(visits_hist: &HashMap<char, i64>) -> Result<String> {
    let mut output = String::new();
    for (state, number_of_visits) in visits_hist {
        output.push(*state);
        output.push_str(&format!(" - {}\n", *number_of_visits));
    }
    Ok(output)
}

fn get_sample_transition_probs() -> Vec<TransitionProb> {
    vec![
        TransitionProb{origin:'a', dest: 'a', jump_prob: 0.5},
        TransitionProb{origin:'a', dest: 'b', jump_prob: 0.275},
        TransitionProb{origin:'a', dest: 'c', jump_prob: 0.225},
        TransitionProb{origin:'b', dest: 'a', jump_prob: 0.15},
        TransitionProb{origin:'b', dest: 'b', jump_prob: 0.8},
        TransitionProb{origin:'b', dest: 'c', jump_prob: 0.05},
        TransitionProb{origin:'c', dest: 'a', jump_prob: 0.25},
        TransitionProb{origin:'c', dest: 'b', jump_prob: 0.25},
        TransitionProb{origin:'c', dest: 'c', jump_prob: 0.5},
    ]
}

fn transition_prob_list_to_map(transition_list: &Vec<TransitionProb>) -> Result<HashMap<char, HashMap<char, f64>>> {
    let mut res = HashMap::<char, HashMap<char, f64>>::new();
    for tp in transition_list {
        let origin_entry = res.entry(tp.origin).or_insert(HashMap::new());
        let dest_entry = origin_entry.entry(tp.dest).or_insert(0f64);
        *dest_entry = tp.jump_prob;
    }
    Ok(res)
}

fn main() -> Result<()> { 
    println!(r#"
    .  .      .            __ .         
    |\/| _.._.;_/ _ .  ,  /  `|_  _.*._ 
    |  |(_][  | \(_) \/   \__.[ )(_]|[ )
                                        
    .  .       ,       __       .   
    |\/| _ ._ -+- _   /  ` _.._.| _ 
    |  |(_)[ ) | (/,  \__.(_][  |(_)
                                    "#);
    let transitions_list = get_sample_transition_probs();
    let (random_walk, hist) = monte_carlo_run('a', &transitions_list, 200)?;
    let hist_string = visits_histogram_to_string(&hist)?;
    println!("Histogram \n\n{}", hist_string);

    println!("Random Walk");
    println!("\ny = States from bottom to top a = 1, b = 2, c = 3");
    Chart::new(280, 60, 0f32, random_walk.len() as f32).lineplot(&Shape::Steps(&random_walk)).display();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transition_prob_list_to_map() -> Result<()> {
        let transition_list = get_sample_transition_probs();
        let transition_map = transition_prob_list_to_map(&transition_list)?;
        
        for tp in &transition_list {
            assert!(transition_map.contains_key(&tp.origin) && transition_map.contains_key(&tp.dest));
        } 
        Ok(())
    }

    #[test]
    fn test_next_step() -> Result<()> {
        let mut count  = 0;
        let mut curr_state = 'a';

        let transition_list = get_sample_transition_probs();
        let transition_map = transition_prob_list_to_map(&transition_list)?;
        
        loop {
            count += 1;
            curr_state = next_state(curr_state, &transition_map)?; 
            assert!(transition_map.contains_key(&curr_state));
            if count > 100 {
                break;
            }
        }
        Ok(())
    }

    #[test]
    fn test_visits_histogram_to_string() -> Result<()> {
        let mut mock_visits_hist = HashMap::new();
        for c in "aabbbcccc".chars() {
            let val = mock_visits_hist.entry(c).or_insert(0i64);
            *val += 1i64;
        }
        let for_display = visits_histogram_to_string(&mock_visits_hist)?;
        
        assert!(for_display.contains("a - 2"));
        assert!(for_display.contains("b - 3"));
        assert!(!for_display.contains("c - 3"));
        
        Ok(())
    }
}
