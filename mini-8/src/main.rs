use std::io;

fn main() {
    let choices = ["rock", "paper", "scissors"];
    let mut player_choice = String::new();

    println!("Let's play Rock, Paper, Scissors!");
    println!("Please enter your choice (rock, paper, or scissors): ");

    io::stdin()
        .read_line(&mut player_choice)
        .expect("Failed to read line");

    let player_choice = player_choice.trim().to_lowercase();

    if !choices.contains(&&player_choice[..]) {
        println!("Invalid choice, please choose rock, paper, or scissors");
        return;
    }

    let computer_choice = choices[rand::random::<usize>() % 3];

    println!("You chose {}", player_choice);
    println!("The computer chose {}", computer_choice);

    if player_choice == computer_choice {
        println!("It's a tie!");
    } else if (player_choice == "rock" && computer_choice == "scissors")
        || (player_choice == "paper" && computer_choice == "rock")
        || (player_choice == "scissors" && computer_choice == "paper")
    {
        println!("You win!");
    } else {
        println!("The computer wins!");
    }
}
