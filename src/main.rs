use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?} {:?}", action, item);

    // create a new instance of our Todo struct
    let mut todo = Todo::new().expect("Initialization of db failed");

    // add an item to our todo list
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else {
        println!("Action not recognized");
    }
}

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // open db.txt
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        // read the db.txt content into a String
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // allocate an empty HashMap
        let mut map = HashMap::new();

        // loop through each line in the file
        for line in content.lines() {
            // split and bind the values to key and val
            let mut values = line.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // insert the key-val pair into hashmap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        // return Ok with our Todo struct
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }
}
