use rpassword::read_password;
use std::env::set_var;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub user: String,
    pub pass: String,
}

pub fn parse_items(objects: Vec<&str>) -> Vec<Vec<String>> {
    let mut reading = false;
    let mut parsed: Vec<Vec<String>> = vec![];
    let mut current_object: Vec<String> = vec![];

    objects.iter().for_each(|obj| match *obj {
        "  {" => {
            reading = true;
        }
        "  }," => {
            reading = false;
            parsed.push(current_object.clone());
        }
        _ => {
            if reading {
                current_object.push(String::from(*obj));
            }
        }
    });

    parsed
}

fn clean_string(input: String) -> String {
    if input.starts_with("\"") {
        if input.ends_with(",") {
            input
                .strip_prefix("\"")
                .unwrap()
                .strip_suffix("\",")
                .unwrap();
        } else {
            input
                .strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .unwrap();
        }
    }
    input.trim().to_string()
}

pub fn build_item(object: Vec<String>) -> Item {
    let mut name = "";
    let mut user = "";
    let mut pass = "";

    object.iter().for_each(|line| {
        if line.contains("\"name\":") {
            let n = line.split(":").collect::<Vec<_>>()[1].trim();
            if n.starts_with("\"") {
                name = n
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\",")
                    .unwrap()
                    .trim();
            } else {
                name = n.strip_suffix(",").unwrap();
            }
        } else if line.contains("\"username\":") {
            let u = line.split(":").collect::<Vec<_>>()[1].trim();
            if u.starts_with("\"") {
                user = u
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\",")
                    .unwrap()
                    .trim();
            } else {
                user = u.strip_suffix(",").unwrap();
            }
        } else if line.contains("\"password\":") {
            let p = line.split(":").collect::<Vec<_>>()[1].trim();
            if p.starts_with("\"") {
                pass = p
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\",")
                    .unwrap()
                    .trim();
            } else {
                pass = p.strip_suffix(",").unwrap();
            }
        }
    });

    Item {
        name: name.to_string(),
        user: user.to_string(),
        pass: pass.to_string(),
    }
}

pub fn unlock() {
    println!("Enter password: ");
    let pass = read_password().expect("Failed to get password");
    set_var("BW_PASSWORD", pass);

    let output = Command::new("bw")
        .arg("unlock")
        .arg("--passwordenv")
        .arg("BW_PASSWORD")
        .output()
        .expect("Failed to unlock");

    let result: Vec<&str> = str::from_utf8(&output.stdout)
        .expect("Failed to get output as string")
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|line| line.contains("export"))
        .collect();

    let key: Vec<_> = result[0].split("\"").collect();
    set_var("BW_SESSION", key[1]);
    // TODO: CLEAR ENVIRONMENT VARIABLE
    // TODO: ZEROIZE PASSWORD
}

pub fn list_items() -> String {
    let listed = Command::new("bw")
        .arg("list")
        .arg("items")
        .arg("--pretty")
        .output()
        .expect("Failed to list items");

    let result = str::from_utf8(&listed.stdout).expect("Failed to get result as str");
    let items = String::from(result);

    items
}
