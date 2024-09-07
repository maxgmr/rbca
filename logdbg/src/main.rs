use std::{fs::File, io::Read};

use pretty_assertions::assert_eq;

fn main() {
    let matches = clap::Command::new("logdbg")
        .arg(
            clap::Arg::new("file_path")
                .help("Path to the log file to load")
                .required(true),
        )
        .arg(
            clap::Arg::new("compare_log")
                .help("The name of the log to compare")
                .required(true),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file_path").unwrap();
    let compare_log_name = matches.get_one::<String>("compare_log").unwrap();

    let mut file_data = String::new();
    File::open(file_path)
        .and_then(|mut f| f.read_to_string(&mut file_data))
        .unwrap();

    let mut compare_data = String::new();
    File::open(format!("logdbg/correct_logs/{compare_log_name}"))
        .and_then(|mut f| f.read_to_string(&mut compare_data))
        .unwrap();

    let file_lines: Vec<&str> = file_data.split('\n').collect();
    let cmp_lines: Vec<&str> = compare_data.split('\n').collect();

    if file_lines.is_empty() {
        eprintln!("Log @ {file_path} is empty.");
        return;
    }

    for i in 0..cmp_lines.len() {
        if file_lines[i] != cmp_lines[i] {
            if i > 1 {
                println!("{}", file_lines[i - 2]);
            }
            if i > 0 {
                println!("{}", file_lines[i - 1]);
            }
            println!("MISMATCH @ LINE {}!", i + 1);
            assert_eq!(file_lines[i], cmp_lines[i]);
            return;
        }

        if (i + 1) == file_lines.len() {
            println!("All {} lines of your log match with the correct log, but the correct log keeps going, meaning your log ends too early!\nLast 5 lines:\n_______", file_lines.len());
            println!("{}", file_lines[i - 4]);
            println!("{}", file_lines[i - 3]);
            println!("{}", file_lines[i - 2]);
            println!("{}", file_lines[i - 1]);
            println!("{}", file_lines[i]);
            return;
        }
    }
    println!("OK!");
}
