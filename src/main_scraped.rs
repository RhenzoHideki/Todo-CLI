// Personal Project doing a Todo List
// What do I need for it ?
// - TODO:  Opening File 
// - TODO: Reading File 
// - TODO: Writing File
// - TODO: Saving  File
// - TODO: Closing File
// - TODO: Create a Struct (content , status)
// - TODO: Save Struct
// - TODO: Read Struct 
// - Done task

use std::fs::OpenOptions;
use std::{env, io};
use std::io::{Write, Read, BufRead};
use std::str;




fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = OpenOptions::new().read(true)
                                            .write(true)
                                            .create(true)
                                            .open("/home/rhenzoubuntu/eng/RustStudy/to-do_list/foo.txt")?;

    
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let mut file = OpenOptions::new().read(true)
                                            .write(true)
                                            .create(true)
                                            .truncate(true)
                                            .open("/home/rhenzoubuntu/eng/RustStudy/to-do_list/foo.txt")?;

    loop { 
        println!("Please enter some text: ");
        let mut s = String::new();

        io::stdin().read_line(&mut s).expect("Failed to read line");
        match s.trim() {
            "read" =>{
                println!("{}",data);
            }
            "write"=>{
                std::io::stdin().lock().read_line(&mut s).expect("failed to readline");
                data.push_str(&s);
            }
            "quit"=>{
                file.write_all(data.as_bytes())?;
                break;
            }
            "remove"=>{
                std::io::stdin().lock().read_line(&mut s).expect("failed to readline");
                data = data.replace(&s,"");
                file.write(data.as_bytes())?;
            }
            _ => println!("Error opcode"),
        }
    }
    Ok(())
}
