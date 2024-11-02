use inquire::Select;
use std::process::{exit, Command};
use git2::Repository;

fn main() {
    println!("Welcome to the Abstracto!");

    let options = vec!["Foundry Non-Diamond", "Foundry Diamond"];
    let framework = Select::new("Choose your contract type:", options)
        .prompt()
        .expect("Failed to read input");

    match framework {
        "Foundry Non-Diamond" => setup_foundry(),
        "Foundry Diamond" => setup_foundry_diamond(),
        _ => {
            eprintln!("Invalid selection.");
            exit(1);
        }
    }
}

fn setup_foundry() {
    println!(
        "Setting up an Account Abstraction project using Foundry (Non Diamond Implementation)..."
    );
    
    Command::new("git")
        .args([
            "clone",
            "https://github.com/blockfuselabs/foundry_account_abstraction.git",
        ])
        .status()
        .expect("Failed to initialize account abstraction project.");

    println!("Stopped!");
}

fn setup_foundry_diamond() {
    println!("Setting up an Account Abstraction project using Foundry (Diamond Implementation)...");

    Command::new("git")
        .args([
            "clone",
            "https://github.com/blockfuselabs/foundry_diamond_account_abstraction.git",
        ])
        .status()
        .expect("Failed to initialize account abstraction project.");

    println!("Stopped!");
}
