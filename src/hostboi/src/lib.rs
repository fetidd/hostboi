use std::fs::{self, File};
use std::fmt;
use std::io::Read;
use regex::Regex;

pub fn get_favorites() -> Result<Vec<String>, String> {
    let hosts = read_hosts()?;
    let fav_pattern = Regex::new(r"#FAV\[([a-zA-Z0-9]*)\]").expect("bad fav pattern");
    let mut favorites = vec![];
    for line in hosts {
        if fav_pattern.is_match(&line) {
            let line_fav = match fav_pattern.captures(&line) {
                Some(cap) => cap.get(1).unwrap(),
                None => continue,
            };
            let new_fav = String::from(line_fav.as_str());
            if !favorites.contains(&new_fav) {
                favorites.push(new_fav);
            }
        }
    }
    Ok(favorites)
}

pub fn swap(box_number: i32) -> Result<(), String> {
    if box_number < 1 {
        return Err("must be 1 or above".into())
    }
    backup()?;
    let mut curr_hosts = read_hosts()?;
    let mut managed = false;
    let managed_pattern = Regex::new(r"#MANAGED").expect("bad managed pattern");
    let unmanaged_pattern = Regex::new(r"#/MANAGED").expect("bad endmanaged pattern");
    let swap_pattern = Regex::new(r"#SWAP").expect("bad swap pattern");
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
                let box_number = box_number.to_string();
                ip[2] = &box_number;
                let ip = ip.join(".");
                split_line[0] = &ip;
                line.clear();
                line.push_str(&split_line.join(" "));
            } else if !line.starts_with("#") {
                line.insert(0, '#');
            }
        }
    }
    write_hosts(&curr_hosts)?;
    Ok(())
}

pub fn favorite(selector: String) -> Result<(), String> {
    backup()?;
    let mut curr_hosts = read_hosts()?;
    let mut managed = false;
    let managed_pattern = Regex::new(r"#MANAGED").expect("bad managed pattern");
    let unmanaged_pattern = Regex::new(r"#/MANAGED").expect("bad endmanaged pattern");
    let fav_pattern = Regex::new(r"#FAV\[([a-zA-Z0-9]*)\]").expect("bad fav pattern");
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
            if fav_pattern.is_match(&line) {
                // check the captured fav value against the passed one and uncomment
                let line_fav = match fav_pattern.captures(&line) {
                    Some(cap) => cap.get(1).unwrap(),
                    None => continue,
                };
                if line.starts_with("#") && selector == line_fav.as_str() {
                    line.remove(0);
                } else if !line.starts_with("#") && selector != line_fav.as_str() {
                    line.insert(0, '#');
                }
            } else if !line.starts_with("#") {
                line.insert(0, '#');
            }
    }
    }
    write_hosts(&curr_hosts)?;
    Ok(())
}

pub fn write_hosts(curr_hosts: &Vec<String>) -> Result<(), String> {
    let hosts_path = get_hosts_path()?;
    let new_hosts = curr_hosts.join("\n");
    fs::write(&hosts_path, new_hosts).or_else(|e| Err(e.to_string()))
}

pub fn restore_backup() -> Result<(), String> {
    let hosts_path = get_hosts_path()?;
    let mut backup_path = hosts_path.clone();
    backup_path.push_str(".backup");
    fs::copy(&backup_path, &hosts_path)
        .and_then(|_| Ok(()))
        .or_else(|e| Err(Error::RestoreFail(e.to_string()).to_string()))
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
