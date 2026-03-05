use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    println!("Enter `ls` command");
    io::stdin().read_line(&mut buf)?;
    println!("command entered : {}", buf.trim());
    Command::new(buf.trim())
        .arg("-l")
        .spawn()
        .expect("`ls` command not found!!!");
    Ok(())
}
