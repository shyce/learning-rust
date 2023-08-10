mod todo;
mod helper;
mod journal;

fn main() {
    loop {
        let choice = main_menu();

        match choice.as_str() {
            "1" => todo::main(),
            "2" => journal::main(),
            "exit" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn main_menu() -> String {
    helper::create_menu("Shyce's Toolkit", &vec![
        "Todo App",
        "Journal",
    ])
}
