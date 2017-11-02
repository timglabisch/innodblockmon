extern crate mysql;
#[macro_use]
extern crate clap;
extern crate chrono;


mod config;
mod server;

use config::Config;
use server::Server;
use clap::{Arg, App};
use std::{thread, time};
use chrono::prelude::*;


fn main() {

    let matches = App::new("MySql InnoDB Lock Monitor")
        .version("1.0")
        .author("Tim Glabisch. <mysql-lock-monitor@tim.ainfach.de>")
        .about("MySql InnoDB Lock Monitor")
        .arg(Arg::with_name("user")
            .help("user")
            .long("user")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("pass")
            .help("pass")
            .long("pass")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("host")
            .help("host")
            .long("host")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("port")
            .help("port")
            .long("port")
            .required(true)
            .takes_value(true))
        .get_matches();


    let config = Config {
        user: Some(matches.value_of("user").expect("user not given").to_string()),
        pass: matches.value_of("pass").and_then(|x|Some(x.to_string())),
        host: matches.value_of("host").expect("host not given").to_string(),
        port: matches.value_of("port").expect("port not given").to_string()
    };

    println!("config: {:#?}", &config);

    println!("waiting for locks...");

    
    loop {

        match Server::new(config.clone()) {
            Err(_) => {
                println!("could not create connection.");
                continue;
            },
            Ok(mut server) => {
                loop {

                    match self::handle(&mut server) {
                        Err(e) => {
                            println!("Error: {:#?}", e);
                            break;
                        }, 
                        Ok(_) => {}
                    };
                
                }
                println!("reconnect ...");
                thread::sleep(time::Duration::from_secs(1));
            }
        }

    }
}

pub fn handle(server : &mut Server) -> Result<(), String> {
    loop {
        thread::sleep(time::Duration::from_secs(1));

        let transactions = server.get_transactions();

        if transactions.len() == 0 {
            continue;
        }

        println!("-{}-Found {} Transactions that are locking ---\n{:#?}", Utc::now(), transactions.len(), transactions);
    }

    Ok(())
}