use std::env;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;

// changelog
// change backwards logic

fn print_help() {
    println!("usage: notes <option> [text..]");
    println!("options:");
    println!("    n    create new note");
    println!("    l    show all current notes");
    println!("    d    select note to delete");
    println!("    h    print help page");
}

fn check_and_create_file() -> Result<PathBuf, io::Error> {
    if let Some(home_dir) = dirs::home_dir() {
        let file_path = home_dir.join(".notes_storage_file"); // surely nobody has an exisiting
                                                              // file with that name..
        if fs::metadata(&file_path).is_ok() {
        } else {
            let _ = File::create(&file_path)?;
            println!("new note file created");
        }
        Ok(file_path)
    } else {
        // If home directory cannot be retrieved, return an error
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    }
}

fn new_note(args: &[String], mut file: &File) -> std::io::Result<()> {
    let mut note: Vec<String> = Vec::new();
    for arg in args.iter().skip(2) {
        if !arg.trim().is_empty() {
            note.push(arg.to_owned());
        }
    }
    if note.is_empty() {
        return Ok(());
    }
    let content = note.join(" ");
    file.write_all(content.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    Ok(())
}

fn list_notes(mut file: &File) -> std::io::Result<()> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

fn delete_note(mut file: &File) -> std::io::Result<()> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<String> = contents.lines().map(String::from).collect();
    if contents.trim().is_empty() {
        eprintln!("file is empty");
        return Ok(());
    }
    for (i, line) in lines.iter().enumerate() {
        println!("{i}: {line}", i = i + 1);
    }
    print!("Select note to delete: ");
    stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("not a valid number");
            return Ok(());
        }
    };

    if num < 0 || !(num <= lines.len()) {
        eprintln!("not valid");
        return Ok(());
    }
    lines.remove(num - 1);
    let content = lines.join("\n");
    file.seek(io::SeekFrom::Start(0))?;
    //remove any existing content
    file.set_len(0)?;
    file.write_all(content.as_bytes())?;
    if content.trim().is_empty() {
        return Ok(());
    }
    file.write_all("\n".as_bytes())?;
    Ok(())
}

fn main() -> ExitCode {
    let file_path = match check_and_create_file() {
        Ok(fp) => fp,
        Err(_) => panic!("ERROR: filesystem behaving weird"), // shouldn't fail ever
    };

    let file = match OpenOptions::new().read(true).append(true).open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("ERROR: can't open file"),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print_help();
        return ExitCode::SUCCESS;
    }
    if args[1] == "n" {
        match new_note(&args, &file) {
            Ok(()) => (),
            Err(err) => panic!("ERROR: {err}"),
        }
    } else if args[1] == "l" {
        match list_notes(&file) {
            Ok(()) => (),
            Err(err) => panic!("ERROR: {err}"),
        }
    } else if args[1] == "d" {
        match delete_note(&file) {
            Ok(()) => (),
            Err(err) => panic!("ERROR: {err}"),
        }
    } else if args[1] == "h" {
        print_help();
    } else {
        eprintln!("invalid argument");
        print_help();
    }
    ExitCode::SUCCESS
}
