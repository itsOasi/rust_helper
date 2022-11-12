use crate::helper::{cli, event, file};
use crate::node::{self, FieldArray};

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

fn close_app(){
    event::freeze();
}

pub fn bobs_burgers_app()->node::Node{
    let mut scene = node::Node::new(); // the root of the app
    let chef = node::AccessRule::new("chef", 
    Some(true), Some(true), Some(true)); // Bob can read, write, or do anything with this rule applied
    let daughter = node::AccessRule::new("daughter", 
    Some(true), Some(false), Some(false)); // Tina can only read anything with this rule applied
    let customer = node::AccessRule::new("customer", 
    Some(true), Some(false), Some(true)); // Teddy can read and do anything with this rule applied
    
    
    scene.add_rule(chef);
    scene.add_rule(daughter);
    scene.add_rule(customer);
    
    pub fn login_func(args: node::FieldArray)->node::FieldArray{ // demo purposes only, please use secure logic and best practices in your code
        let users = vec![ // represents a database of users
            "Bob", "Tina", "Teddy"
        ];
        let mut res = node::FieldArray::new();
        let name =  args.get_field("username").unwrap().to_string();  
        if name == users[0]{
            let _id = node::Field::new("id", node::FieldType::Text("123456".to_string()), None);
            let _roles = node::Field::new("roles", node::FieldType::TextArray(vec!["chef".to_string()]), None);
            res.add_field(_id);
            res.add_field(_roles);
        }
        if name == users[1]{
            let _id = node::Field::new("id", node::FieldType::Text("123456".to_string()), None);
            let _roles = node::Field::new("roles", node::FieldType::TextArray(vec!["daughter".to_string()]), None);
            res.add_field(_id);
            res.add_field(_roles);
        }
        if name == users[2]{
            let _id = node::Field::new("id", node::FieldType::Text("123456".to_string()), None);
            let _roles = node::Field::new("roles", node::FieldType::TextArray(vec!["customer".to_string()]), None);
            let cash = node::Field::new("cash", node::FieldType::Number(25), None);
            res.add_field(_id);
            res.add_field(_roles);
            res.add_field(cash)
        }
        res
    }

    let login = node::Field::new("login", node::FieldType::Func(login_func), None);
    scene.add_field(login);
    
    let price = node::Field::new("price",node::FieldType::Number(20), None);
    scene.add_field(price);

    pub fn purchase_burger(args: node::FieldArray)->node::FieldArray{
        /*
            requires cash, price
        */
        let mut res = node::FieldArray::new();
        let mut cash_field = args.get_field("cash").unwrap();
        let mut cash = cash_field.to_usize();
        let price = args.get_field("price").unwrap().to_usize();
        cash -= price;
        cash_field.write(
            &args.get_field("roles").unwrap().to_string_arr(), 
            node::FieldType::Number(cash));
        res.add_field(cash_field);
        res
    }
    let purchase = node::Field::new("purchase_burger", node::FieldType::Func(purchase_burger), None);
    scene.add_field(purchase);
    
    scene
}

pub fn check_price(){
    let bobs_burgers = bobs_burgers_app();
    let mut Bob = node::User::login("Bob", "12345", bobs_burgers);
    
    Bob.read_field("price");
    Bob.write_field("price", node::FieldType::Number(40));
}

pub fn cant_change_price(){
    let bobs_burgers = bobs_burgers_app();
    let mut Tina = node::User::login("Tina", "12345", bobs_burgers);
    Tina.read_field("price");
    Tina.write_field("price", node::FieldType::Number(20));
    
}

pub fn purchase_burger(){
    let bobs_burgers = bobs_burgers_app();
    let mut Teddy = node::User::login("Teddy", "12345", bobs_burgers); // create Teddy
    let mut args = FieldArray::new();
    args.add_field(Teddy.get_field("cash"));
    args.add_field(Teddy.get_field("price"));
    Teddy.do_action("purchase_burger", args)
}

pub fn menu_test(){    let mut menu = cli::menu::Menu::new("what would you like to say?");

    let m1 = cli::menu::MenuItem::new("hello", write_hello);
    let m2 = cli::menu::MenuItem::new("read file", read_file);
    let m3 = cli::menu::MenuItem::new("close app", close_app);


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