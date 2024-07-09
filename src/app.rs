use rpassword::read_password;
use std::env::set_var;
use std::process::{Command, Stdio};
use std::str;

pub struct App {
    pub accounts: Vec<Account>,
    pub selected: usize,
    pub active_account: Option<Account>,
}

// TODO: fix permissions
#[derive(Clone, Debug)] // TODO: FIX THIS!
pub struct Account {
    pub id: String,
    pub name: String,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl Account {
    fn get_pass(self) -> Option<String> {
        self.pass
    }
}

impl App {
    pub fn new() -> App {
        App {
            accounts: vec![],
            selected: 0,
            active_account: None,
        }
    }

    pub fn fetch_items(&mut self, accounts: Vec<Account>) {
        self.accounts = accounts;
    }

    pub fn update_active_account(&mut self, index: usize) {
        self.active_account = Some(get_account(&self.accounts[index].id)); // TODO: FIX THIS!
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

pub fn list_accounts() -> String {
    let bw = Command::new("bw")
        .args(["list", "items"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn bw");

    let bw_out = bw.stdout.expect("failed to open bw stdout");

    let jq = Command::new("jq")
        .arg(".[]|{id:.id,name:.name,user:.login.username,pass:.login.password}")
        .stdin(Stdio::from(bw_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn jq");

    let output = jq.wait_with_output().expect("failed to wait on jq");

    let result = str::from_utf8(&output.stdout).expect("failed to get result as str");
    let items = String::from(result);

    items
}

fn clean_string(input: &str) -> String {
    let dirty = input.trim();
    let mut stripped = "";
    if dirty.starts_with("\"") {
        if dirty.ends_with(",") {
            stripped = dirty
                .strip_prefix("\"")
                .unwrap()
                .strip_suffix("\",")
                .unwrap();
        } else {
            stripped = dirty
                .strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .unwrap();
        }
    }

    let clean = stripped.trim().to_string();
    clean
}

pub fn get_account(id: &String) -> Account {
    let bw = Command::new("bw")
        .args(["get", "item", &id])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn bw");

    let bw_out = bw.stdout.expect("failed to open bw stdout");

    let jq = Command::new("jq")
        .arg(".|{id:.id,name:.name,user:.login.username,pass:.login.password}")
        .stdin(Stdio::from(bw_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn jq");

    let output = jq.wait_with_output().expect("failed to wait on jq");

    let result: Vec<&str> = str::from_utf8(&output.stdout)
        .expect("failed to get result as str")
        .split("\n")
        .collect();

    let id = clean_string(result[1].split(":").collect::<Vec<&str>>()[1]);
    let name = clean_string(result[2].split(":").collect::<Vec<&str>>()[1]);
    let user = clean_string(result[3].split(":").collect::<Vec<&str>>()[1]);
    let pass = clean_string(result[4].split(":").collect::<Vec<&str>>()[1]);

    Account {
        id,
        name,
        user: Some(user),
        pass: Some(pass),
    }
}

pub fn parse_items(items: String) -> Vec<Account> {
    let mut reading = false;
    let mut accounts: Vec<Account> = vec![];
    let mut current_object: Vec<String> = vec![]; // TODO: ZEROIZE?
    let objects: Vec<&str> = items.split("\n").collect();

    objects.iter().for_each(|obj| match *obj {
        "{" => {
            reading = true;
        }
        "}" => {
            reading = false;
            let id = clean_string(current_object[0].split(":").collect::<Vec<_>>()[1]);
            let name = clean_string(current_object[1].split(":").collect::<Vec<_>>()[1].trim());

            accounts.push(Account {
                id: id.to_string(),
                name: name.to_string(),
                user: None,
                pass: None,
            });

            current_object = vec![];
        }
        _ => {
            if reading {
                current_object.push(String::from(*obj));
            }
        }
    });

    accounts
}
