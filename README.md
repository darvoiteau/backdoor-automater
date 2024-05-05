# Firewall-Checker
A tool coded in Rust for use in the post-exploitation phase of an offensive security system.
This tool enables you to create backdoors on a Linux system effortlessly.

## Features

- Creation of a poisoned systemd service containing a backdoor
- Poisoning an existing systemd service
- Creation of a backdoor in the .bashrc file
- Create a backdoor with cron to schedule it
- Some backdoors can be installed using the built-in tools of a Linux system.

## Installation

Project Installation Instructions.

```bash
git clone https://github.com/darvoiteau/backdoor-automater.git
cd backdoor-automater
cargo build --release
cd target/release
./backdoorautomater
```
**** or ****

Download the release here to get the binary executable: <a href="https://github.com/darvoiteau/backdoor-automater/releases/tag/backdoorautomater">backdoor-automater releases</a>

chmod +x backdoorautomater-xxxxx
./backdoorautomater-xxxxx --help


## Usage

It's a post-operation tool. It needs to be run on the machine you've attacked, and to which you already have access. <br>
In parallel, you must have a netcat listening on your attacker machine.<br><br>

To run the tool just put this command:<br><br>

./backdoorautomater-xxxxx<br><br>

and answer to questions to install the backdoor:<br><br>


:::red
Choose what kind of backdoor you want to install !<br><br>

1 Create new systemd malicious service  -> <code style="color : red">Need to be root</code><br><br>

2 Infect existing systemd service  -> <font color="red">Need to be root and can broke the service. Don't use it in production servers !!!</font><br><br>

3 Infect .bashrc of current user  -> Infect .bashrc file of the current user. You will have a reverse shell when the user try to connect on the machine<br><br>

4 Schedule malicious script with crontab  -> Schedule a reverse shell<br>

## Disclaimer
This tool is proposed to help you within a legal framework and to ensure the consent of all parties involved in your offensive tests.<br><br>
This tool is also offered for educational purposes, to help you learn more about post-exploitation and backdoors.<br><br>
In the event of illegal use of this tool, the author cannot be held responsible.<br>
