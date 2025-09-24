use std::process::Command;
use std::process::exit;
use std::process::ExitCode;
use std::os::unix::process::CommandExt;
use std::env;
use std::fs;
use std::io::{stdin, Stdout, stdout, Write};
use std::fs::File;
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use serde_json::Value;

fn read_config() -> Value {

    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Environment variable $HOME not found or invalid: {}", e);
            exit(2);
        }
    };

    let cont = match fs::read_to_string(home_dir.clone() + "/.config/tuimenu/list.json") {
        Ok(x) => x,
        Err(_) => {
            let path = home_dir + "/.config/tuimenu";

            //CHECK FOR DIR AND MAKE IF NOT THERE

            let tuimenu_dir = Path::new(&path);

            if !tuimenu_dir.exists() {

                match fs::create_dir(path.clone()) {
                    Ok(_) => {},
                    Err(e) => eprintln!("\x1b[31mError\x1b[0m creating directory {}:\n{}", path.clone(), e),
                }

            }

            match File::create(path.clone() + "/list.json") {
                Ok(mut x) => {
                    let _ = x.write_all(b"");
                        eprintln!("\x1b[31mError\x1b[0m {} is empty\n\n    Please vist https://github.com/achester88/tuimenu to find out more\n", path + "/list.json");
                },
                Err(e) => {
                        eprintln!("\x1b[31mError\x1b[0m creating file {}:\n{}", path.clone() + "/list.json", e);
                }
            }
            exit(1);
        }
    };
    
    match serde_json::from_str(&cont) {
        Ok(x) => return x,
        Err(e) => {
            eprintln!("\n\x1b[31mError\x1b[0m reading tuimenu config:\n    {}", e);
            exit(1);
        }
    };
}

fn print_list(stdout: &mut RawTerminal<Stdout>, len: usize, programs: &Value, selected: u16) {
    let mut i: u16 = 0;

    while (i as usize) < (len+1) {
        let cmd: String;
        let mut desc = String::from("");

        
        let mut sel = vec!(' ', ' ');
        let mut background = "\x1b[40m";
        if selected == (i+1) {
            sel = vec!('>', '<');
            background = "\x1b[30;107m";
        }

        if (i as usize) < len {
            cmd = String::from(&programs[i as usize]["cmd"].as_str().unwrap().to_string());
            desc = String::from(&programs[i as usize]["desc"].as_str().unwrap().to_string());
        } else {
            cmd = String::from("Quit(press q)");
            background = "\x1b[0m";
            i += 1;
        }

        let line  = format!(" {} {}\x1b[1m{}\x1b[0;90m - {}\x1b[0m {}", sel[0], background, cmd, desc, sel[1]);
        
        let _ = write!(
        stdout, "{}{}",
        termion::cursor::Goto(1, i+3),
        line,
        );

        stdout.flush().unwrap();

        i += 1;
    }

    let _ = write!(stdout, "{}({}/{})", 
        termion::cursor::Goto(1, i+4), 
        if (selected as usize) > len {0} else {selected}, 
        len
    );
    stdout.flush().unwrap();
}

fn main() -> ExitCode {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-v" {
        const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
        println!("tuimenu v{}", VERSION.unwrap_or("unknown"));
        exit(0);
    }

    let programs = read_config();
    let mut selected = 1;

    println!("{}", programs[0]["cmd"].as_str().unwrap());

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();


    let _ = write!(stdout, "{}{}[Launch Program]\n{}", 
        termion::clear::All,
        termion::cursor::Goto(1,1),
        termion::cursor::Hide
        );

    let len = programs.as_array().unwrap().len();

    print_list(&mut stdout, len, &programs, selected);     
    
    for k in stdin.keys() {

        match k.as_ref().unwrap() {
            Key::Up   => { 
                if selected > 1  {selected -= 1}
                print_list(&mut stdout, len, &programs, selected);    
            }
            Key::Down => { 
                if selected < ((len+1) as u16) {selected += 1}
                print_list(&mut stdout, len, &programs, selected);
            },
            Key::Char('\n') => {
                if ((selected-1) as usize) == len {
                    break
                } else {
                    let cmd = &programs[(selected-1) as usize];

                    let mut command = Command::new(cmd["cmd"].as_str().unwrap());
                    match cmd["args"].as_array() {
                        Some(x) => {
                            let opargs: Vec<Option<&str>> = x.into_iter().map(Value::as_str).collect();
                            let args: Vec<&str> = opargs.into_iter().map(|x| x.unwrap()).collect();
                            
                            command.args(args);
                            eprintln!("Failed to exec: {}", cmd["cmd"]);
                        },
                        None => {}
                    }

                    write!(stdout, "\n{}", termion::cursor::Show).unwrap();
                    stdout.flush().unwrap();
                    stdout.suspend_raw_mode().unwrap();
                    let _ = command.exec();
                }
            }
            Key::Char('q') => break,
            _ => {}
        }

    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
    stdout.suspend_raw_mode().unwrap();
    return ExitCode::SUCCESS;
}
