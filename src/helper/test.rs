use crate::helper::{cli, event, file};

use super::thrd;

fn write_hello(){   
    if file::write("hello_world.txt", "hello world".as_bytes()){
        event::log("data written!");
    }
    else{
        event::log_err("something went wrong", 2);
    }
}

fn read_file(){
    let mut msg = String::new();
    file::read("hello_world.txt", &mut msg);
    event::log(msg.as_str());
    
}

pub fn menu_test(){    let mut menu = cli::menu::Menu::new("what would you like to say?");

    let m1 = cli::menu::MenuItem::new("hello", write_hello);
    let m2 = cli::menu::MenuItem::new("read file", read_file);
	let m3= cli::menu::MenuItem::new("thread test", thread_test);

    menu.add_item(m1, None);
    menu.add_item(m2, None);
    menu.add_item(m3, None);
    
    let running = true;

    while running{
        menu.poll();
    }
}

fn return_number()->Vec<usize>{
    vec![32]
}

pub fn thread_test(){
    let msg = thrd::new_usize_vec(return_number);
    event::log(format!("{msg:?}").as_str())
}