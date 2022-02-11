use std::fs::File;
use std::io::ErrorKind;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut db: HashyMap = read_database();

    let args = std::env::args().collect::<Vec<String>>();
    match args[1].as_str().to_lowercase().as_str() {
        "set_database" => { overwrite_default(args[2].clone()).ok().unwrap(); },
        "print_database" => { println!("{:?}", read_default()); }
        "read" => { db = read_database(); }
        "insert" => { 
            db.add_value(str_to_hash(args[2].clone().as_str()), args[3].clone()); 
            save_database(db); 
        }
        "delete" => { 
            db.remove_value(str_to_hash(args[2].clone().as_str())); 
            save_database(db); 
        }
        "show" => { 
            let values = db.find_values(str_to_hash(args[2].clone().as_str()));
            for value in values {
                println!("{:?}", value);
            }
        }
        "list" => {
            for value in db.table {
                println!("{:?}", value.1);
            }
        }
        _ => { eprintln!("[ERROR]: Unrecognized pattern: {:?}.", args[1]); }
    }
}

struct HashyMap {
    table: Vec<(u32, Vec<String>)>,
}

impl HashyMap {
    fn add_value(&mut self, k: u32, v: String) {
        if self.table.iter().map(|x| x.0).collect::<Vec<u32>>().contains(&k) {
            self.table.iter_mut().find(|x| x.0 == k).unwrap().1.push(v);
        } else {
            self.table.push((k, vec!(v)));
        }
    }

    fn remove_value(&mut self, k: u32) {
        if self.table.iter().map(|x| x.0).collect::<Vec<u32>>().contains(&k) {
            self.table.remove(self.table.iter().position(|x| x.0 == k).unwrap());
        }
    }

    fn find_values(&self, k: u32) -> Vec<String> {
        let mut vec: Vec<String> = vec!();
        if self.table.iter().map(|x| x.0).collect::<Vec<u32>>().contains(&k) {
            vec = self.table[self.table.iter().position(|x| x.0 == k).unwrap()].1.clone();
        }
        vec
    }

    fn from_string(db: String) -> HashyMap {
        let mut string = db.split('|').collect::<Vec<&str>>().iter().map(|x| x.split(':').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        let mut data: Vec<(u32, Vec<String>)> = vec!();
        for mut elem in string {
            let key = elem.remove(0);
            let value = elem;
            data.push((key.parse::<u32>().unwrap(), value.iter().map(|x| x.to_string()).collect::<Vec<String>>()));
        };
        let hashmap = HashyMap {
            table: data,
        };
        hashmap
    }

    fn to_string(&self) -> String {
        let mut string_vec: Vec<String> = vec!();
        for elem in &self.table {
            let mut string: String = elem.0.to_string();
            for value in &elem.1 {
                string += ":";
                string += value.as_str();
            }
            string_vec.push(string);
        };
        let mut string_final: String = string_vec.remove(0);
        for elem in string_vec {
            string_final += "|";
            string_final += elem.as_str();
        };
        string_final
    }
}

fn read_database() -> HashyMap {
    let mut file = open_database();
    let mut string = "".to_string();
    file.read_to_string(&mut string);
    if string == "" {
        return HashyMap {
            table: vec!()
        }
    } else {
        return HashyMap::from_string(string);
    }
}

fn save_database(db: HashyMap) {
    let result = File::create(read_default());
    if result.is_err() {
        panic!("[FATAL ERROR]: Program lacks permissions to open files.");
    };
    let mut file = result.unwrap();
    file.write(db.to_string().as_bytes());
}

fn open_database() -> File {
    let path = read_default();
    let mut result = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(path);

    if result.is_err() {
        panic!("[FATAL ERROR]: Program lacks permissions to open files.")
    };
    let mut file = result.ok().unwrap();
    file
}

fn str_to_hash(str: &str) -> u32 {
    //convert the characters to a list of bytes
    let bytes = str.chars().map(|x| x as u32).collect::<Vec<u32>>();
    let mut sum: u32 = 0;
    for byte in bytes {
        //circularly bitshift the sum
        sum = sum.rotate_right(1);
        //add the next byte
        sum += byte;
        //truncate to u32
        sum &= 0xffffffff;
    }
    sum
}

fn read_default() -> String {
    let mut file = open_default();
    let mut contents: String = "".to_string();
    file.read_to_string(&mut contents);
    if contents == "" {
        overwrite_default("database".to_string());
        contents = "database".to_string();
    };
    contents
}

fn overwrite_default(content: String) -> Result<(), ErrorKind> {
    let result = File::create("default");
    if result.is_err() {
        panic!("[FATAL ERROR]: Program lacks permissions to open files.");
    };
    let mut file = result.unwrap();
    if content == "" {
        return Result::Err(ErrorKind::AddrNotAvailable);
    }
    file.write(content.as_bytes());
    Ok(())
}

fn open_default() -> File {
    let mut result = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open("default");
    if result.is_err() {
        eprintln!("[FATAL ERROR]: Program lacks permissions to open files.");
    };
    result.unwrap()
}