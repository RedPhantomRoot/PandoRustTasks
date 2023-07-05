use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_banner();
        operate_file();
        print_help();
    } else if args[1] == "-s" || args[1] == "--send" {
        if args.len() != 4 {
            print_help();
            return;
        }
        let address = operate_file();
        let password = read_password();
        send_form_request(address, &args[2], &args[3], &password).unwrap();
    } else if args[1] == "-l" || args[1] == "--list" {
        if args.len() != 2 {
            print_help();
            return;
        }
        let address = operate_file();
        let password = read_password();
        list_form_request(address, &password).unwrap();
    } else if args[1] == "-f" || args[1] == "--finish" {
        if args.len() != 3 {
            print_help();
            return;
        }
        let address = operate_file();
        let password = read_password();
        finish_form_request(address, &args[2], &password).unwrap();
    } else if args[1] == "-r" || args[1] == "--reset" {
        if args.len() != 2 {
            print_help();
            return;
        }
        let address = operate_file();
        let password = read_password();
        reset_form_request(address, &password).unwrap();
    } else if args[1] == "-c" || args[1] == "--change" {
        if args.len() != 2 {
            print_help();
            return;
        }
        change_address();
    } else {
        print_help();
    }
}

fn print_banner() {
    println!(r"
############################################################################
_____                _       _____           _ _______        _        
|  __ \              | |     |  __ \         | |__   __|      | |       
| |__) |_ _ _ __   __| | ___ | |__) |   _ ___| |_ | | __ _ ___| | _____ 
|  ___/ _` | '_ \ / _` |/ _ \|  _  / | | / __| __|| |/ _` / __| |/ / __|
| |  | (_| | | | | (_| | (_) | | \ \ |_| \__ \ |_ | | (_| \__ \   <\__ \
|_|   \__,_|_| |_|\__,_|\___/|_|  \_\__,_|___/\__||_|\__,_|___/_|\_\___/

                                Welcome to PandoRustTasks 1.21!
############################################################################
    ");
}

fn print_help() {
    println!("Usage: pandorusttasks [command] [argument]");
    println!("Command:");
    println!("  -s, --send [key] [task]: send a task to server");
    println!("  -l, --list: list all tasks");
    println!("  -f, --finish [key]: finish a task");
    println!("  -r, --reset: reset all tasks");
    println!("  -c, --change: change server's domain(or IPAddress)");
    println!("Example:");
    println!("  pandorusttasks -s breakfast \"Need to buy milk.\"");
    println!("  pandorusttasks --list");
    println!("  pandorusttasks -f breakfast");
    println!("  pandorusttasks --reset");
    println!("  pandorusttasks -c")
}

fn file_exist() -> bool {
    let file_path = "server.txt";
    let path = Path::new(file_path);

    path.exists()
}

fn make_file() {
    let mut address = String::new();
    println!("Please enter server's domain name(or IPAddress): ");
    std::io::stdin().read_line(&mut address).unwrap();

    let mut server_file = File::create("server.txt").expect("creation failed");
    server_file.write_all(&mut address.trim().as_bytes()).expect("write failed");

    println!("[+]Created server.txt");
}

fn read_file() -> String {
    let mut server_file = File::open("server.txt").unwrap();
    let mut address = String::new();
    server_file.read_to_string(&mut address).unwrap();

    return address;
}

// create server.txt if it doesn't exist
// return domain or ip address
fn operate_file() -> String {
    if !file_exist() {
        make_file();
    }
    let address = read_file();
    //println!("{}", address);

    return address;
}

fn change_address() -> String {
    let address = operate_file();
    println!("Your current domain is {}", address);
    let mut new_address = String::new();
    println!("Please enter server's domain name(or IPAddress): ");
    std::io::stdin().read_line(&mut new_address).unwrap();

    let mut server_file = File::create("server.txt").expect("creation failed");
    server_file.write_all(&mut new_address.trim().as_bytes()).expect("write failed");

    println!("You updated the domain to {}", new_address);
    return new_address;
}

// necessary for someone else not to send a task to your server
fn read_password() -> String {
    println!("Please enter your server password: ");
    let password = rpassword::read_password().unwrap();
    //println!("Your password is {}", password);

    return password;
}

#[tokio::main]
async fn send_form_request(domain: String, task_key: &str, task: &str, password: &str) -> reqwest::Result<()> {
    let params = [("taskKey", task_key.to_string()), ("task", task.to_string()), ("password", password.to_string())];
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/send", domain);
    //println!("{}", url);
    let res = client.post(url).form(&params).send().await?.text().await?;
    println!("Response from server: {}", res);
    Ok(())
}

#[tokio::main]
async fn reset_form_request(domain: String, password: &str) -> reqwest::Result<()> {
    let params = [("password", password.to_string())];
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/reset", domain);
    //println!("{}", url);
    let res = client.post(url).form(&params).send().await?.text().await?;
    println!("Response from server: {}", res);
    Ok(())
}

#[tokio::main]
async fn finish_form_request(domain: String, task_key: &str, password: &str) -> reqwest::Result<()> {
    let params = [("taskKey", task_key.to_string()), ("password", password.to_string())];
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/finish", domain);
    //println!("{}", url);
    let res = client.post(url).form(&params).send().await?.text().await?;
    println!("Response from server: {}", res);
    Ok(())
}

#[tokio::main]
async fn list_form_request(domain: String, password: &str) -> reqwest::Result<()> {
    let params = [ ("password", password.to_string())];
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/list", domain);
    //println!("{}", url);
    let res = client.post(url).form(&params).send().await?.text().await?;
    let object: HashMap<String, Value> = serde_json::from_str(&res).unwrap();
    for (key, value) in &object {
        if let Value::String(s) = value {
            println!("{}: {}", key, s);
        } else {
            // Handle other value types or do nothing
        }
    }
    Ok(())
}