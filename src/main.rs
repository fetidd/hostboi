use clap::Parser;
use regex::Regex;
use std::io::Read;
use std::{
    fmt,
    fs::{self, File},
};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    swap: Option<i32>,
    #[arg(short, long)]
    favorite: Option<String>,
}

#[derive(Debug)]
enum Error {
    HostsUnavailable,
    ReadFail(String),
    BackupFail(String),
    WriteFail(String),
    RestoreFail(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostsUnavailable => write!(f, "cannot find hosts file"),
            Self::ReadFail(e) => write!(f, "failed to read hosts file: {}", e.to_string()),
            Self::BackupFail(e) => write!(f, "failed to backup hosts: {}", e.to_string()),
            Self::WriteFail(e) => write!(f, "failed to write hosts: {}", e.to_string()),
            Self::RestoreFail(e) => write!(f, "failed to restore backup: {}", e.to_string()),
        }
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    validate_args(&args)?;
    backup()?;
    let mut curr_hosts = read_hosts()?;
    modify_hosts(&mut curr_hosts, &args)?;
    match write_hosts(&curr_hosts) {
        Ok(()) => Ok(()),
        Err(e) => {
            restore_backup()?;
            Err(Error::WriteFail(e.to_string()).to_string())
        }
    }
}

fn validate_args(args: &Args) -> Result<(), String> {
    if !args.swap.is_some() && !args.favorite.is_some() {
        return Err("need to specify at least one arg".into());
    } else if args.swap.is_some() && args.favorite.is_some() {
        return Err("can't swap and set favorite at the same time".into());
    }
    Ok(())
}

fn get_hosts_path() -> Result<String, String> {
    match std::env::consts::OS {
        "linux" => Ok(String::from("/etc/hosts")),
        "windows" => Ok(String::from(r"C:\Windows\System32\drivers\etc\hosts")),
        _ => Err(Error::HostsUnavailable.to_string()),
    }
}

fn backup() -> Result<(), String> {
    let hosts_path = get_hosts_path()?;
    let mut backup_path = hosts_path.clone();
    backup_path.push_str(".backup");
    fs::copy(&hosts_path, &backup_path)
        .and_then(|_| Ok(()))
        .or_else(|e| Err(Error::BackupFail(e.to_string()).to_string()))
}

fn read_hosts() -> Result<Vec<String>, String> {
    let mut buffer = String::new();
    let mut hosts = File::open(get_hosts_path()?)
        .or_else(|e| Err(Error::ReadFail(e.to_string()).to_string()))?;
    hosts
        .read_to_string(&mut buffer)
        .or_else(|e| Err(Error::ReadFail(e.to_string()).to_string()))?;
    let lines: Vec<String> = buffer.split("\n").map(|s| s.to_owned()).collect();
    Ok(lines)
}

fn modify_hosts(curr_hosts: &mut Vec<String>, args: &Args) -> Result<(), String> {
    let mut managed = false;
    let managed_pattern = Regex::new(r"#MANAGED").expect("bad managed pattern");
    let unmanaged_pattern = Regex::new(r"#/MANAGED").expect("bad endmanaged pattern");
    let swap_pattern = Regex::new(r"#SWAP").expect("bad swap pattern");
    let fav_pattern = Regex::new(r"#FAV\[([a-zA-Z0-9]*)\]").expect("bad fav pattern");
    let ip_pattern = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").expect("bad ip pattern");
    for line in &mut *curr_hosts {
        if line.is_empty() {
            continue;
        }
        if unmanaged_pattern.is_match(line.as_str()) {
            managed = false;
        }
        if managed_pattern.is_match(line.as_str()) {
            managed = true;
            continue;
        }
        if managed {
            if args.swap.is_some() {
                if swap_pattern.is_match(line.as_str()) {
                    if line.starts_with("#") {
                        line.remove(0);
                    }
                    let split_line = line.clone();
                    let mut split_line: Vec<&str> = split_line
                        .split(r" ")
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if split_line.len() == 0 || !ip_pattern.is_match(&split_line[0]) {
                        continue;
                    }
                    let mut ip: Vec<&str> = split_line[0].split(".").collect();
                    let new_val = args.swap.unwrap().to_string();
                    ip[2] = &new_val;
                    let ip = ip.join(".");
                    split_line[0] = &ip;
                    line.clear();
                    line.push_str(&split_line.join(" "));
                } else if !line.starts_with("#") {
                    line.insert(0, '#');
                }
            }
            if args.favorite.is_some() {
                if fav_pattern.is_match(&line) {
                    // check the captured fav value against the passed one and uncomment
                    let arg_fav = args.favorite.as_ref().unwrap();
                    let line_fav = match fav_pattern.captures(&line) {
                        Some(cap) => cap.get(1).unwrap(),
                        None => continue,
                    };
                    if line.starts_with("#") && arg_fav == line_fav.as_str() {
                        line.remove(0);
                    } else if !line.starts_with("#") && arg_fav != line_fav.as_str() {
                        line.insert(0, '#');
                    }
                } else if !line.starts_with("#") {
                    line.insert(0, '#');
                }
            }
        }
    }
    println!("{curr_hosts:#?}");
    Ok(())
}

fn write_hosts(curr_hosts: &Vec<String>) -> Result<(), String> {
    let hosts_path = get_hosts_path()?;
    let new_hosts = curr_hosts.join("\n");
    fs::write(&hosts_path, new_hosts).or_else(|e| Err(e.to_string()))
}

fn restore_backup() -> Result<(), String> {
    let hosts_path = get_hosts_path()?;
    let mut backup_path = hosts_path.clone();
    backup_path.push_str(".backup");
    fs::copy(&backup_path, &hosts_path)
        .and_then(|_| Ok(()))
        .or_else(|e| Err(Error::RestoreFail(e.to_string()).to_string()))
}
