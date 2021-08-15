fn main() {
    println!("Hello, world!");
    let str_content = String::from("testString");
    print(&str_content);
    print_string(str_content);
    //print_string(str_content);// use of moved String, string goes out of scope
    //print(&str_content); // goes out of scope
}

fn print(var_str: &str) {
    println!("The value of the string is {}", var_str)
}

fn print_string(var_str: String) {
    println!("The value of the string is {}", var_str)
}
