#[derive(Debug)]
struct KeyPress(String, char); //tuple struct

#[derive(Debug)]
struct MouseClick{x:i64, y:i64} //clasic struct

//define webEvent enum to use data from structs
#[derive(Debug)]
enum WebEvent{
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick),
}

fn main() {

    let click = MouseClick{x :100, y: 100};
    println!("Mouse clicked on: {}, {}", click.x, click.y);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed:{}{}",keys.0,keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);

    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

    let web_case = we_load;

    match web_case {
        WebEvent::WEClick(_) => println!("Mouse click event"),
        WebEvent::WEKeys(_) => println!("Key pressed"),
        WebEvent::WELoad(_) =>println!("Loading page")

    }
}
