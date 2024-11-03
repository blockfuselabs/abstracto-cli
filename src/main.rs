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

    let repo_url = "https://github.com/blockfuselabs/foundry_account_abstraction.git";
    let local_path = "./AccountAbstraction-test";

    match Repository::clone(repo_url, local_path) {
        Ok(_) => { 
            println!("Successfully initialize Account-Abstraction !");
            println!("");
            println!("=========================");
            
            println!("cd {}", local_path);
            println!("cd ./src");
            println!("Running forge build .....");
            println!("=========================");
            Command::new("forge")
            .args([
                "build"])
            .status()
            .expect("Failed to build account abstraction project.");
        }
        ,
        Err(e) => eprintln!("Failed to clone repository: {}", e),
    }

    println!("Done!");
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
