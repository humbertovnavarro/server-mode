use std::{fs::File, env, process::Command};

fn main() {
    let desktop_services = "/etc/server-mode/desktop-services.json";
    let desktop_services = File::open(desktop_services).unwrap();
    let desktop_services: Vec<String> = serde_json::from_reader(desktop_services).unwrap();
    let server_services = "/etc/server-mode/server-services.json";
    let server_services = File::open(server_services).unwrap();
    let server_services: Vec<String> = serde_json::from_reader(server_services).unwrap();
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        "desktop" => {
            start_services(desktop_services);
            stop_services(server_services);
        },
        "server" => {
            stop_services(desktop_services);
            start_services(server_services);
        }
        &_ => help(),
    }
}

fn stop_services(services: Vec<String>) {
    for service in services {
        match Command::new("systemctl")
        .args(["stop", service.as_str()])
        .spawn() {
            Ok(_) => {
                continue;
            },
            Err(err) => println!("{:?}", err)
        }
    }
}

fn start_services(services: Vec<String>) {
    for service in services {
        match Command::new("systemctl")
        .args(["start", service.as_str()])
        .spawn() {
            Ok(_) => {
                continue;
            },
            Err(err) => println!("{:?}", err)
        }
    }
}


fn help() {
    println!("available options are desktop and server")
}