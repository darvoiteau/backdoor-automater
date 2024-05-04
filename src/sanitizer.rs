use std::net::Ipv4Addr;
use std::io;
use regex::Regex;
use std::process::Command;

use crate::Param;

pub fn sanitize_param_hub(mut param: Param, kind: u8) {
    param.source_ip = sanitize_ip(param.source_ip);
    param.service_name = sanitize_service(param.service_name);
    param.port = sanitize_port(param.port);
    param.time = sanitize_time(param.time);
    param.path = sanitize_path(param.path);
    param.choice = sanitize_choice(param.choice, kind);
    param.script_name = sanitize_script(param.script_name);
}

fn sanitize_ip(ip: String) -> String {
    let _ip_sanitized: Ipv4Addr = ip.parse().expect("Invalid IP !");

    ip
}

fn sanitize_service(service_name: String) -> String {
    let re_special_char = Regex::new(r"[^a-zA-Z0-9.]+").unwrap();

    if re_special_char.is_match(&service_name) {

        panic!("Invalid character in service name !\n");

    }

    service_name
}

fn sanitize_port(port: String) -> String {
    let re_special_alphabet_char = Regex::new(r"[^0-9]+").unwrap();
    let error_convert = "Error ! Given port is invalid ! \n";

    if re_special_alphabet_char.is_match(&port) {
        panic!("Invalid character in port\n");
    }
    let port_u16: u16 = port.trim().parse().expect(&(error_convert.to_owned() + &port));

    if port_u16 >= 65535 {
        panic!("Error ! Given port is not in the TCP Range (1-65536)\n");
    }
    else if port_u16 == 0 {
        panic!("Error ! Given port cannot be 0 !\n");
    }

    port
}

fn sanitize_choice(choice: u8, kind: u8) -> u8 {
    if choice > 4 {
        panic!("Error ! Please choose an option.\n");
    }
    else if choice == 0 {
        panic!("Error ! Please choose an option.\n");
    }

    if kind == 3 || kind == 4 {
        if choice > 3 {
            panic!("Error ! Please choose an option.\n");
        }
    } 

    choice
}

fn sanitize_time(time: String) -> String {
    let re_invalid_char = Regex::new(r"[^0-9/]+").unwrap();

    if re_invalid_char.is_match(&time) {
        panic!("Error! Invalid time to schedule a cron.\n");
    }

    time
}

fn sanitize_path(path: String) -> String {
    let re_invalid_special_char = Regex::new(r"[^a-zA-Z0-9./\-_]+").unwrap();

    if re_invalid_special_char.is_match(&path) {
        panic!("Error ! Invalid charactere in your absolute directory path.\n");
    }

    
    
    
    path
    
}

fn sanitize_script(script_name: String) -> String {
    let re_invalid_special_char = Regex::new(r"[^a-zA-Z0-9.\-_]+").unwrap();

    if re_invalid_special_char.is_match(&script_name) {
        panic!("Error ! Invalid charactere in the script name file !\n");
    }

    script_name
}

pub fn sanitize_system_command(err: &String) {
    if err.is_empty(){
        

    }
    else {
        panic!("Error: {} \n", err);
    }

}

pub fn sanitize_return_fn (result: Result<(), io::Error>) {
    match result {
        Ok(()) => {

        }
        Err(err) => {
            panic!("Error! Something was wrong ! : {}\n", err);
        }

    }
    

}

pub fn sanitize_command_exist(command: &String) -> bool {
    let result: bool;
    let command_v = ["command -v ", &command].concat();
    let output = Command::new("sh")
    .arg("-c")
    .arg(&command_v)
    .output()
    .expect("Failed to execute command");

    let output_string = String::from_utf8_lossy(&output.stdout).to_string();

    if output_string.is_empty() {
        result = false;
    }
    else{
        result = true;
    }

    result
}

pub fn sanitize_ex_stderr(err: &String){
    if err.contains("Error") {
        panic!("Error!: {}", err);
    }
}

pub fn sanitize_service_exist(service: &String) {
    let systemd_command = ["systemctl status ", service].concat();
    let output = Command::new("sh")
    .arg("-c")
    .arg(&systemd_command)
    .output()
    .expect("Failed to execute command");

    let err = String::from_utf8_lossy(&output.stderr).to_string();


    if err.is_empty(){
        panic!("Error ! The service already exist on your system !\n")

    }
}