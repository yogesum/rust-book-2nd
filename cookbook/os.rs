#!/usr/bin/env run-cargo-script
// cargo-deps: regex

extern crate regex;

use std::process::{Command, Stdio};
use regex::Regex;

#[derive(Debug, PartialEq, Default, Clone)]
struct Commit {
    hash: String,
    message: String,
}

fn os_cmd() -> Result<(), std::io::Error> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        panic!("Command executed with failing error code");
    }

    let pattern = Regex::new(
        r"(?x)
        ([0-9a-fA-F]+) # commit hash
        (.*)           # The commit message"
    ).unwrap();

    String::from_utf8(output.stdout).unwrap()
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

use std::collections::HashSet;
use std::io::Write;

fn stdin_cmd() -> Result<(), std::io::Error> {
    let mut child = Command::new("python").stdin(Stdio::piped())
        .stderr(Stdio::piped())    
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!").unwrap()
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout).unwrap();
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr).unwrap();
        panic!("External command failed:\n {}", err)
    }
}

fn piped_cmd() -> Result<(), std::io::Error> {
    let directory = std::env::current_dir()?;
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;

        du_output_child.wait()?;

        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("head")
                .args(&["-n", "10"])    
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()?;

            let head_stdout = head_output_child.wait_with_output()?;

            sort_output_child.wait()?;

            println!(
                "Top 10 biggest files and directories in '{}':\n{}",
                directory.display(),
                String::from_utf8(head_stdout.stdout).unwrap(),
            );
        }
    }

    Ok(())
}

fn std_redirect() -> Result<(), std::io::Error> {
    use std::fs::File;
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

fn live_processing() -> Result<(), std::io::Error> {
    use std::io::{BufRead, BufReader};

    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture standard output.").unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}


fn main() {
    os_cmd().unwrap();
    stdin_cmd().unwrap();
    piped_cmd().unwrap();
    std_redirect().unwrap();
    live_processing().unwrap();
}