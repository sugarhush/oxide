use chrono::Local;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::process::Command;

use crate::gui::gui;

mod gui;

fn main() -> io::Result<()> {
    gui();
    let mut session_buf = String::new();
    loop {
        println!("Enter `ls` command");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let cmd = buf.trim();
        create_logs(cmd, &mut session_buf);

        if cmd == "exit" {
            println!("bye bye...");
            write_logs(&mut session_buf)?;
            break;
        }

        println!("command entered : {}", cmd);
        Command::new(cmd)
            .arg("-lah")
            .status()
            .expect("`ls` command not found!!!");
        buf.clear()
    }
    Ok(())
}

fn create_logs(cmd: &str, buffer: &mut String) {
    let dt = Local::now();
    let date_string = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    buffer.push_str(&date_string);
    buffer.push_str(" ");
    buffer.push_str(cmd);
    buffer.push_str("\n");
}

fn write_logs(buf: &mut String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("logs.txt")?;
    writeln!(file, "{}", buf)?;
    println!("{}", buf);
    Ok(())
}
