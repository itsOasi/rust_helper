use std::process::{Command, Output};

pub mod menu{
    // a simple and intuitive system to capture and respont to user input in the commandline
    use crate::helper::event;

    pub struct MenuItem{
        name: String,
        callback: fn()
    }

    impl MenuItem{
        pub fn new(name: &str, callback: fn()) -> Self{
            MenuItem { name: name.to_string(), callback }
        }

        pub fn prompt(&mut self){
            let name = &self.name;
            println!("{name}");
        }

        pub fn test(&mut self, query: String){
            let name = self.name.to_string();
            let matched = query.eq(&name);
            if matched {
                (self.callback)();
            }
        }
    }

    pub struct Menu{
        prompt: String,
        items: Vec<MenuItem>
    }

    impl Menu {
        pub fn new(prompt: &str)->Self{
            Menu { prompt: prompt.to_string(), items: Vec::new() }
        }

        pub fn add_item(&mut self, item: MenuItem, order: Option<usize>){
            self.items.insert(
                order.unwrap_or(self.items.len()), 
                item
            );
        }

        pub fn poll(&mut self){
            for i in &mut self.items{ // list menu items
                i.prompt();
            }

            event::log(self.prompt.as_str()); // show prompt
            
            let mut query = String::new(); // variable to hold response
            std::io::stdin().read_line(&mut query).unwrap(); // get response
            for i in &mut self.items{ // loop through menu items
                    i.test(query.trim().to_string()); // lets item test the query and executes if matched
            }
            
            event::log("continue..."); // let user know that we are waiting for input
            std::io::stdin().read_line(&mut "".to_string()).unwrap(); // continue after user presses enter
        }
    }
}

pub fn os(exe:&str, args: Vec<String>) -> Output{
    let output = Command::new(exe)
    .args(args)
    .output()
    .expect("command failed");
    output
}