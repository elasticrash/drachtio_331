use chrono::{prelude::*, Duration};
use rsip::{SipMessage, StatusCode};
use std::env;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::thread::sleep;
use udp_polygon::{
    config::{Address, Config, FromArguments},
    Polygon,
};

mod ack;
mod invite;
mod util;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let delay = args[1].parse::<i64>().unwrap();

    let udp_config = Config::from_arguments(
        vec![Address {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 5061,
        }],
        Some(Address {
            ip: IpAddr::V4(Ipv4Addr::new(172, 29, 0, 10)),
            port: 5060,
        }),
    );

    let mut polygon = Polygon::configure(udp_config);
    let rx = polygon.receive();

    let origin = "+443332222";
    let destination = "+111111111";

    let mut last_execution = Utc::now();
    loop {
        print!(".");
        let utc_now = Utc::now();
        let elapsed_time = utc_now.signed_duration_since(last_execution);
        let maybe_msg = rx.try_recv();

        if let Ok(..) = maybe_msg {
            println!(
                "{}",
                String::from_utf8_lossy(&maybe_msg.clone().unwrap()).to_string()
            );

            let msg = SipMessage::try_from(maybe_msg.unwrap()).unwrap();

            match msg {
                SipMessage::Request(request) => match request.method() {
                    _ => println!("request method: {}", request.method),
                },
                SipMessage::Response(response) => match response.status_code {
                    StatusCode::BusyHere => {
                        let ack = ack::create(
                            response.clone(),
                            origin.to_string(),
                            destination.to_string(),
                        );
                        let clone_msg = ack.clone().to_string();
                        log(clone_msg);

                        polygon.send(ack.to_string().as_bytes().to_vec());
                    }
                    _ => println!("<> {}", response.status_code),
                },
            }
        }

        if elapsed_time >= Duration::seconds(delay) {
            last_execution = Utc::now();
            let invite = invite::create(origin.to_string(), destination.to_string());
            let clone_msg = invite.clone().to_string();
            log(clone_msg);

            polygon.send(invite.to_string().as_bytes().to_vec());
        }
        let _ = std::io::stdout().flush();
        sleep(std::time::Duration::from_millis(100));
    }
}

fn log(msg: String) {
    let print: Vec<&str> = msg.split("\r\n").collect();
    for line in print {
        println!("{}", line);
    }
}
