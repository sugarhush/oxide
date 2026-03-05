use std::process::Command;

fn main() {
    println!("Hello, oxide!!!");
    let s1: &str = "ls";
    println!("ls - list directory");
    Command::new(s1)
        .arg("-l")
        .spawn()
        .expect("`ls` command not found!!!");
}
