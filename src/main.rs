use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use bincode::deserialize_from;
use std::io;
use std::panic;

fn find_optimal (scramble: &str, memos: &Vec<Vec<i32>>, sets: &HashMap<String, Vec<i64>>) {
    // calculates the turn amount for each of the "memos"
    let s: Vec<i32> = scramble.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    let mut moves = vec![0;58829];
    let mut counter = 0;
    for m in memos {
        let mut c = 0;
        for i in 0..14 {
            c += m[i] * s[i];
        }
        moves[counter] = ((c % 12) + 12) % 12;
        counter+=1;
    }
    
    // goes through each of the pin sets and uses the calculated "memos" to find out which set is
    // the best
    let mut best_moves = 100;
    let mut best_mset_ticks = 1000;
    let mut best_mset = "idk";

    let mut best_ticks = 1000;
    let mut best_tset_moves = 100;
    let mut best_tset = "idk";
    let tick_counts = [0,1,2,3,4,5,6,5,4,3,2,1];
    'outer: for (key, value) in sets.iter() {
        let mut move_count = 0;
        let mut ticks = 0; 
        for &i in value {
            if moves[i as usize] != 0 {
                move_count += 1;
                ticks += tick_counts[moves[i as usize] as usize];
            }
            if move_count >= best_tset_moves && ticks >= best_mset_ticks {
                continue 'outer;
            }
        }
        if ticks < best_ticks || (ticks == best_ticks && move_count < best_tset_moves) {
            best_tset = key;
            best_ticks = ticks;
            best_tset_moves = move_count;
        }
        if move_count < best_moves || (move_count == best_moves && ticks < best_mset_ticks) {
            best_mset = key;
            best_moves = move_count;
            best_mset_ticks = ticks;
        }
    } 
    
    // everything else in below this is for printing out the solution
    if best_ticks == best_mset_ticks {
        println!("{} moves ({} ticks) is optimal:",best_moves,best_mset_ticks);
    }
    else {
        println!("{} moves is move optimal:",best_moves);
    }
    let mut solution = String::from("");
    let mut solution_moves: Vec<&str> = best_mset.split_whitespace().collect();
    let move_names = ["0+","1-","2-","3-","4-","5-","6+","5+","4+","3+","2+","1+"];
    if let Some(array) = sets.get(best_mset) {
        let mut i = 0;
        for element in array.iter() {
            if solution_moves[i as usize] == "y2" {
                i += 1;
                solution += "y2 ";
            }
            if moves[*element as usize] != 0 {
                solution.push_str(solution_moves[i]);
                solution.push_str(move_names[moves[*element as usize] as usize]);
                solution.push_str(" ");
            }
            i += 1;
        }
    }
    println!("{}",solution.trim());
    if best_ticks != best_mset_ticks {
        println!("{} ticks is tick optimal:",best_ticks);
        solution = String::from("");
        solution_moves = best_tset.split_whitespace().collect();
        if let Some(array) = sets.get(best_tset) {
            let mut i = 0;
            for element in array.iter() {
                if solution_moves[i as usize] == "y2" {
                    i += 1;
                    solution += "y2 ";
                }
                if moves[*element as usize] != 0 {
                    solution.push_str(solution_moves[i]);
                    solution.push_str(move_names[moves[*element as usize] as usize]);
                    solution.push_str(" ");
                }
                i += 1;
            }
        }
        println!("{}",solution.trim());
    }
}

fn main() {
    println!("data loading...");
    let file = File::open("matrixRows.bin").unwrap();
    let reader = BufReader::new(file);
    let memos: Vec<Vec<i32>> = deserialize_from(reader).unwrap();
    let file = File::open("pinSets.bin").unwrap();
    let reader = BufReader::new(file);
    let sets: HashMap<String, Vec<i64>> = deserialize_from(reader).unwrap();
    
    println!("data loaded");
    loop {
        println!("enter a scramble state:");
        let mut scramble = String::new();
        io::stdin()
            .read_line(&mut scramble)
            .expect("Failed to read line");
        let result = panic::catch_unwind(|| {
            find_optimal(&scramble, &memos, &sets);
        });
        match result {
            Ok(_) => {}
            Err(_) => {
                println!("Bad input. Please enter 14 space separated numbers. The first nine are the dials on the first face starting at the top left, and the next 5 numbers are the non-corner dials on the back.");
            }
        }
    }
}
