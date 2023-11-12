// Personal Project doing a Todo List
// What do I need for it ?
// - Done:  Opening File 
// - Done: Reading File 
// - Done: Writing File
// - Done: Saving  File
// - DOne: Closing File
// - Done: Create a Struct (content , status)
// - Done: Save Struct
// - Done: Read Struct
// - TODO: Remove from File 
// - Done task

use std::{collections::HashMap, io::Read};
use std::str::FromStr;

struct Todo {
    map: HashMap<String,bool>,
}

impl Todo {
    fn new() -> Result<Todo,std::io::Error>{
        let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let map: HashMap<String,bool> = content
        .lines()
        .map(|line|line.splitn(2,'\t').collect::<Vec<&str>>())
        .map(|v|(v[0],v[1]))
        .map(|(k,v)|(String::from(k),bool::from_str(v).unwrap()))
        .collect();
        Ok(Todo { map })
    
        /*
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert them into HashMap
             map.insert(String::from(key), bool::from_str(val).unwrap());
        }
         */
    }

    fn insert(&mut self,key:String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(),std::io::Error> { 
        let mut content = String::new();
        for(k,v) in self.map{
            let record = format!("{}\t{}\n",k,v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }
    
    fn complete(&mut self, key: &String)-> Option<()>{
        match self.map.get_mut(key){
            Some(v) => Some(*v = false),
            None => None,
        }
    }

}

fn main(){
    let action = std::env::args().nth(1).expect("Please speficy an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}",why),
        }
    }else if action == "complete"{
        match todo.complete(&item){
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save(){
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurred: {}",why),
            }
        }
    } else if action == "remove" {
        match todo.map.remove(&item){
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save(){
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurred: {}",why),
            }
        }
    }
}