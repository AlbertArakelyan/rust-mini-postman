use reqwest::Method;

struct Request {
    url: String,
    method: Method,
}

enum KeyCommand {
    N,
    Q,
    Invalid,
}

fn read_command() -> KeyCommand {
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).unwrap();

    match command.trim() {
        "N" => KeyCommand::N,
        "Q" => KeyCommand::Q,
        _ => KeyCommand::Invalid,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Mini Postman Console App");

    loop {
        println!("Available commands:");
        println!("N - New request, Q - Quit");

        let command = read_command();

        let mut url = String::new();
        println!("Enter URL: ");
        std::io::stdin().read_line(&mut url)
            .expect("Failed to read line."); 
    }       
}
