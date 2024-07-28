use std::env;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;

fn print_help() {
    println!("usage: notes <option> [text..]");
    println!("options:");
    println!("    n    create new note");
    println!("    l    show all current notes");
    println!("    d    select notes to delete"); // example?
    println!("    h    print help page"); // better help page
    println!("    e    print example"); // better help page
}

fn print_example() {
    println!("this app helps you create quick notes in your terminal");
    println!();
    println!("option n -> creates a new note:");
    println!("notes n hello world");
    println!();
    println!("your note got saved, print all your saved notes with the l option");
    println!("you can delete a note or multiple notes with \"notes d\"");
    println!("notes d 2 3 1 -> deletes note 1,2 and 3");
}

fn check_and_create_file() -> Result<PathBuf, io::Error> {
    if let Some(home_dir) = dirs::home_dir() {
        // change this path for a custom file location
        let file_path = home_dir.join(".notes_storage_file"); // uniquefilenamebelike

        match fs::metadata(&file_path).is_ok() {
            true => Ok(file_path),
            false => {
                let _ = File::create(&file_path)?;
                println!("new note file created");
                Ok(file_path)
            }
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    }
}

fn new_note(args: &[String], mut file: &File) -> std::io::Result<()> {
    let mut note: Vec<String> = Vec::new();
    // collecting provided "note" from args
    note.push(
        args.into_iter()
            .skip(2)
            .filter(|arg| !arg.trim().is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<_>>()
            .join(" "),
    );
    if note.is_empty() {
        return Ok(());
    }
    let content: String = note.join(" ");
    file.write_all(content.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    Ok(())
}

fn list_notes(mut file: &File) -> std::io::Result<()> {
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    for line in lines {
        println!("{line}");
    }
    Ok(())
}

fn delete_note(mut file: &File) -> std::io::Result<()> {
    let mut contents: String = String::new();
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

    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    let mut numbers: Vec<usize> = input
        .split(" ")
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .filter(|x| *x <= lines.len() && *x != 0)
        .collect();

    numbers.sort_by(|a, b| b.cmp(a));

    for num in numbers {
        lines.remove(num - 1);
    }

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
    if args.len() == 1 {
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
    } else if args[1] == "e" {
        print_example();
    } else {
        eprintln!("invalid argument");
        print_help();
    }
    ExitCode::SUCCESS
}
