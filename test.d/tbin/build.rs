use std::process::Command;
fn main() {
    assert!(Command::new("mcs")
        .arg("Program.cs")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}
