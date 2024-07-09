use bitwarden_tui::{build_item, list_items, parse_items, unlock, Item};
use std::str;
use std::time::Instant;

fn main() {
    println!("STILL IN TESTING!");
    unlock();
    let now = Instant::now();

    let items = list_items();
    let objects: Vec<&str> = items.split("\n").collect();
    let parsed = parse_items(objects);

    let mut parsed_items: Vec<Item> = vec![];
    for object in parsed {
        parsed_items.push(build_item(object));
    }

    for item in parsed_items {
        println!(
            "Name: {}\nUsername: {}\nPassword: {}",
            item.name, item.user, item.pass
        );
    }

    println!("{}", now.elapsed().as_millis());
}
