use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;
//use dirs;
// notes -arg text
// notes -n neue notiz
// notes -d notiz löschen
// notes -l list
// notes    list
// load text file
// if it doesnt exist, create it,
// ~/home/user/.notes

fn check_and_create_file() -> Result<PathBuf, io::Error> {
    if let Some(home_dir) = dirs::home_dir() {
        let file_path = home_dir.join(".notes_storage");
        if fs::metadata(&file_path).is_ok() {
            //println!("file exist");
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
    // parse line into vec?
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
    println!("{}", &content);
    file.write_all(content.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    println!("note created");

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

    for (i, line) in lines.iter().enumerate() {
        println!("{i}: {line}", i = i + 1);
    }
    print!("Select note to delete: ");
    stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Parse the input into an integer
    //let num: usize = input.trim().parse().expect("Please enter a valid number");

    let num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("not a valid number");
            return Ok(());
        }
    };

    if num > 0 && num <= lines.len() {
        eprintln!("valid number");
    } else {
        eprintln!("not valid");
        return Ok(());
    }

    lines.remove(num - 1);
    let content = lines.join("\n");
    file.seek(io::SeekFrom::Start(0))?;
    // Truncate the file to remove any existing content
    file.set_len(0)?;
    // Write new content to the file
    file.write_all(content.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    // + rewrite it to file
    Ok(())
}

fn main() -> ExitCode {
    let file_path = match check_and_create_file() {
        Ok(fp) => fp,
        Err(_) => panic!("ERROR: file fucked up"),
    };

    let file = match OpenOptions::new().read(true).append(true).open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("ERROR: can't open file"),
    };

    //let mut args: Vec<String> = env::args().collect();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("ERROR: no argument provided");
        return ExitCode::FAILURE;
    }
    // arg 1 => n, d, l
    // arg 2.. => new note, if arg1

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
    } else {
        eprintln!("wrong argument");
    }
    ExitCode::SUCCESS
}
