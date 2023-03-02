use openssl::ssl::{SslConnector, SslMethod};
use std::{io::{BufReader, BufRead}};
use std::io;
use std::net::TcpStream;
use std::io::Write;
use std::fs::File;


fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.len() {
        0 => {
            println!("Usage: webget [url]");
        }
        _ => {
            let get = get_get(&arguments[0]);
            let last = get_last(&arguments[0]);
            let host = get_host(&arguments[0]);
            let protocol = get_protocol(&arguments[0]);
            let safe = get_safe(&arguments[0]);
            let get_message = format!("GET {get}\r\nHost: {host}\r\nConnection: Close\r\n\r\n"); //https://doc.rust-lang.org/std/macro.format.html
            match send_message(&host, 443, &get_message, &last) {
                Ok(_) => println!("Worked"),
                Err(e) => println!("Error {e}"),
            }
            println!("{}", get_message);
        }
    }
}

fn get_get(arg: &str) -> String {
    let splits: Vec<&str> = arg.split("//").collect(); // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let second = splits[1].split("/");
    let mut file = "/".to_owned();
    for part in second.skip(1) {
        file.push_str(&format!("{}", part)); //https://stackoverflow.com/questions/70754394/how-to-push-string-in-string-in-rust#:~:text=You%20cannot%20push%20String%20directly,as_str()%20method
        file.push_str("/");
    }
    file = file[0..file.len()-1].to_owned();
    file.push_str(" HTTP/1.1");
    file.to_owned()
}

fn get_last(arg: &str) -> String {
    arg.split("/").last().unwrap().to_owned()
}

fn get_host(arg: &str) -> String {
    let splits: Vec<&str> = arg.split("//").collect();
    let second: Vec<&str> = splits[1].split("/").collect();
    let host = second[0];
    host.to_owned()
}

fn get_protocol(arg: &str) -> String {
    let splits: Vec<&str> = arg.split("://").collect();
    let protocol = splits[0];
    protocol.to_owned()
}

fn get_safe(arg: &str) -> bool {
    let splits: Vec<&str> = arg.split("://").collect();
    let protocol = splits[0];
    match protocol.len() {
        4 => return false,
        _ => return true,
    }
}


// From Project 5 https://hendrix-cs.github.io/csci320/projects/webget
fn send_message(host: &str, port: usize, message: &str, f: &str) -> io::Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp).unwrap();
    stream.write(message.as_bytes())?;
    let reader = BufReader::new(stream);
    let mut out_ready = false;
    let mut file_out = File::create(f)?;
    for line in reader.lines() {
        let line = line?;
        if out_ready {
            println!("Used: {line}");
            writeln!(file_out, "{}", line)?;
        } else {
            println!("{}", line);
            if line.len() == 0 {
                out_ready = true;
            }
        }
        
    }
    Ok(())
}


