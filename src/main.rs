use clap::Parser;
use std::{fs::{self, File}, fmt};
use std::io::Read;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    swap: Option<i32>,
    #[arg(short, long)]
    favorite: Option<String>
}

#[derive(Debug)]
enum Error {
    HostsUnavailable,
    ReadFail(String),
    BackupFail(String),
    WriteFail(String),
    RestoreFail(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostsUnavailable => write!(f, "cannot find hosts file"),
            Self::ReadFail(e) => write!(f, "failed to read hosts file: {}", e.to_string()),
            Self::BackupFail(e) => write!(f, "failed to backup hosts: {}", e.to_string()),
            Self::WriteFail(e) => write!(f, "failed to write hosts: {}", e.to_string()),
            Self::RestoreFail(e) => write!(f, "failed to restore backup: {}", e.to_string())
        }
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    match args {
        Args {swap: Some(swaparg), favorite: None} => {},
        Args {swap: None, favorite: Some(favarg)} => {},
        _ => Err("Bad arguments".into())
    }
    backup()?;
    let mut curr_hosts = read_hosts()?;
    modify_hosts(&mut curr_hosts, &args)?;
    write_hosts(&curr_hosts).or_else(|e| {
        restore_backup()?;
        Err(Error::RestoreFail(e.to_string()))
    })
}

fn get_hosts_path() -> Result<String, Error> {
    match std::env::consts::OS {
        "linux" => Ok(String::from("./hosts")),
        "windows" => Ok(String::from("./hosts")),
        _ => Err(Error::HostsUnavailable)
    }
}

fn backup() -> Result<(), Error> {
    let mut hosts_path = get_hosts_path()?;
    let mut backup_path = hosts_path.clone();
    backup_path.push_str(".backup");
    fs::copy(&hosts_path, &backup_path)
        .and_then(|_| Ok(()))
        .or_else(|e| Err(Error::BackupFail(e.to_string())))
}

fn read_hosts() -> Result<Vec<String>, Error> {
    let mut buffer = String::new();
    let mut hosts = File::open(get_hosts_path()?).or_else(|e| Err(Error::ReadFail(e.to_string())))?;
    hosts.read_to_string(&mut buffer).or_else(|e| Err(Error::ReadFail(e.to_string())))?;
    let lines: Vec<String> = buffer.split("\n").map(|s| s.to_owned()).collect();
    Ok(lines)
}

fn modify_hosts(mut curr_hosts: &Vec<String>, args: &Args) -> Result<(), Error> {

    Ok(())
}

fn write_hosts(curr_hosts: &Vec<String>) -> Result<(), Error> {

    Ok(())
}

fn restore_backup() -> Result<(), Error> {
    
    Ok(())
}



