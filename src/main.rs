use std::process::Command;
use std::process::exit;
use std::os::unix::process::CommandExt;
use std::env;
use std::fs;
use std::io;
use std::io::{stdin, Stdout, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::raw::RawTerminal;

use serde_json::{Result, Value};

fn read_config() -> Value {

    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Environment variable $HOME not found or invalid: {}", e);
            exit(2);
        }
    };

    let cont = match fs::read_to_string(home_dir + "/.config/tuimenu.json") {
        Ok(x) => x,
        Err(e) => {
            println!("ERR: {}", e);  
            exit(2);
        }
    };
    
    match serde_json::from_str(&cont) {
        Ok(x) => return x,
        Err(e) => {
            println!("ERR: {}", e);
            exit(1);
        }
    };
}

fn print_list(stdout: &mut RawTerminal<Stdout>, len: usize, programs: &Value, selected: u16) {
    let mut i: u16 = 0;

    while (i as usize) < len {
        let cmd = &programs[i as usize];
        let mut sel = vec!(' ', ' ');
        let mut background = "\x1b[40m";
        if selected == (i+1) {
            sel = vec!('>', '<');
            background = "\x1b[30;107m";
        }
        let line  = format!(" {}  {}\x1b[1m{} \x1b[0;90m- {}\x1b[0m {}", sel[0], background, cmd["cmd"].as_str().unwrap(), cmd["desc"].as_str().unwrap(), sel[1]);
        
        write!(
        stdout, "{}{}",
        termion::cursor::Goto(1, (i+3)),
        line,
        );

        stdout.flush().unwrap();

        i += 1;
    }
}

fn main() {
    let programs = read_config();
    let mut selected = 1;

    println!("{}", programs[0]["cmd"].as_str().unwrap());

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();


    write!(stdout, "{}{}[Launch Program]\n{}", 
        termion::clear::All,
        termion::cursor::Goto(1,1),
        termion::cursor::Hide
        );

    let len = programs.as_array().unwrap().len();

    print_list(&mut stdout, len, &programs, selected);     
    
    for k in stdin.keys() {

        match k.as_ref().unwrap() {
            Key::Up   => { 
                if (selected > 1)  {selected -= 1}
                print_list(&mut stdout, len, &programs, selected);    
            }
            Key::Down => { 
                if (selected < (len as u16)) {selected += 1}
                print_list(&mut stdout, len, &programs, selected);
            },
            Key::Char('r') => {
                let mut command = Command::new(&programs[(selected-1) as usize]["cmd"].as_str().unwrap());
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                stdout.flush().unwrap();
                command.exec();
            }
            Key::Char('q') => break,
            _ => {}
        }

    }

    /*loop {
        for name in &programs {
            println!("{}", name);
        }
    }*/
    //let programs: Vec<&str> = content.lines().collect();
    
    //let mut command = Command::new("timr");
    //command.args()
    //command.exec();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
}
