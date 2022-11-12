use crate::helper::file;

use super::cli;

static mut EVENTS: String = String::new(); // event log,  gets compiled into file

pub fn log(msg: &str) { // creates a message with default log level
    let _msg = format!("LOG: {msg}\n");
    println!("{_msg}");
    unsafe{
        EVENTS += _msg.as_str()
    }
}

pub fn log_err(msg: &str, lvl: u8) { // creates a message with specified log level
    let _lvl = match lvl{
        0 => "LOG",
        1 => "WARN",
        2 => "ERR",
        _ => "FATAL"
    };
    let _msg = format!("{_lvl}: {msg}\n");
    println!();
    unsafe{
        EVENTS += _msg.as_str();
    }
}

pub fn freeze(){ // outputs a log file; called at the end of the application
    cli::os("mkdir logs", vec!["".to_string()]);
    unsafe{
        file::write("logs/date.txt", EVENTS.as_bytes());
    }
}
