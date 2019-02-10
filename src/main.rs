use std::env;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;


struct UserInput {
    ip_address: IpAddr,
    threads: u16,
    starting_port: u16, //starting and ending port form a range of ports to scan
    ending_port: u16,
}

impl UserInput {
    fn new(args: &[String]) -> Result<UserInput, &'static str> {
        let ip = args[1].clone();
        let threads = args[2].parse::<u16>().expect("Error encountered while parsing number of threads").clone();
        let starting_port = args[3].parse::<u16>().expect("Error encountered while parsing starting port").clone();
        let ending_port = args[4].parse::<u16>().expect("Error encountered while parsing ending port").clone(); //If user inputs a number larger that 65535 (maximum port), unsigned 16bit int overflows and program panics
        
        match IpAddr::from_str(&ip) { // if IpAddr can't be constructed from user input then it's not a valid ipv4 or ipv6 address
            Ok(ip_address) => return Ok(UserInput {ip_address, threads, starting_port, ending_port}),
            Err(_) => return Err("Invalid IP"),
        }
    }
}

fn scan(tx: Sender<u16>, starting_port: u16, address: IpAddr, num_threads: u16, ending_port: u16) {
    let mut port: u16 = starting_port;
    loop {
        match TcpStream::connect((address, port)) {
            Ok(_) => {
                tx.send(port).unwrap();
            }

            Err(_) => {}
        }

        if (ending_port - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = UserInput::new(&args).expect("Problem parsing arguments");
    let threads = arguments.threads;
    let ip_address = arguments.ip_address;
    let starting_port = arguments.starting_port;
    let ending_port = arguments.ending_port;

    let (tx, rx) = channel();
    for mut i in 0..threads {
        let tx = tx.clone();
        i += starting_port;

        thread::spawn(move || {
            scan(tx, i, ip_address, threads, ending_port);
        });
    }

    let mut output = Vec::new();
    drop(tx);
    for p in rx {
        output.push(p);
    }

    println!("");
    output.sort();
    for v in output {
        println!("{} is open", v);
    }
}
