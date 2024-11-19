#[macro_use]
extern crate serde_derive;

use std::io::Write;
use std::io::{self};
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut miner_addr);
    print!("input difficulty: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut difficulty);
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        let _ = io::stdout().flush();
        choice.clear();
        let _ = io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut sender);
                print!("enter receiver address:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut receiver);
                print!("enter amount:");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generated failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("updated difficulty successfully"),
                    false => println!("failed to update difficulty"),
                }
            }
            4 => {
                let mut new_reward: String = String::new();
                print!("enter new reward: ");
                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("updated reward successfully"),
                    false => println!("failed to update reward"),
                }
            }
            _ => println!("invalid choice, try again"),
        }
    }
}
