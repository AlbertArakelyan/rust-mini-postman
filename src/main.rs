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

fn read_method() -> Method {
    println!("Select request method: ");
    println!("1. GET
        2. POST
        3. PATCH
        4. PUT
        5. DELETE");

    let mut method_key = String::new();
    std::io::stdin().read_line(&mut method_key).expect("Failed to read line.");

    match method_key.trim() {
        "1" => Method::GET,
        "2" => Method::POST,
        "3" => Method::PATCH,
        "4" => Method::PUT,
        "5" => Method::DELETE,
        _ => read_method(),
    }
}

async fn make_request() -> Result<String, reqwest::Error> {
    let method = read_method();

    println!("Enter URL: ");

    let mut url = String::new();
    std::io::stdin().read_line(&mut url).expect("Failed to read line.");    

    let client = reqwest::Client::new();
    let response = client
        .request(method, url.trim())
        .header("User-Agent", "rust-mini-postman")
        .send().await?;
        
    response.text().await
}

fn quit() -> Result<(), std::error::Error> {
    std::process::exit(0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Mini Postman Console App");

    loop {
        println!("Available commands:");
        println!("N - New request, Q - Quit");

        let command = read_command();

        let res = match command {
            KeyCommand::N => make_request(),
            KeyCommand::Q => quit(),
            KeyCommand::Invalid => println!("Invalid command!"),
        };
    }       
}
