use argon2::{self, Config};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
    RngCore,
};
use simple_crypt::{decrypt, encrypt};
use std::env::set_var;
use std::process::{Command, Stdio};
use std::str;
use zeroize::Zeroize;

pub enum CurrentScreen {
    Login,
    Main,
}

pub struct App {
    pub pass_input: String,
    pub clean_input: String,
    pub unlocked: bool,
    pub current_screen: CurrentScreen,
    pub accounts: Vec<Account>,
    pub selected: usize,
    pub active_account: Option<Account>,
    pub pass_copied: bool,
    pub crypto_key: String,
}

// TODO: fix permissions
#[derive(Clone, Debug)] // TODO: FIX THIS!
pub struct Account {
    pub id: String,
    pub name: String,
    pub user: Vec<u8>,
    pub pass: Vec<u8>,
}

impl Account {
    pub fn get_user(self) -> String {
        let user = String::from_utf8(self.user);
        match user {
            Ok(string) => return string,
            Err(_) => return String::from("Error retrieving username"),
        }
    }

    pub fn get_pass(self) -> String {
        let pass = String::from_utf8(self.pass);
        match pass {
            Ok(string) => return string,
            Err(_) => return String::from("Error retrieving password"),
        }
    }
}

impl App {
    pub fn new() -> App {
        App {
            pass_input: String::new(),
            clean_input: String::new(),
            crypto_key: String::new(),
            unlocked: false,
            current_screen: CurrentScreen::Login,
            accounts: vec![],
            selected: 0,
            active_account: None,
            pass_copied: false,
        }
    }

    pub fn unlock(&mut self) {
        set_var("BW_PASSWORD", &self.pass_input);
        self.pass_input = String::new();
        self.pass_input.zeroize();

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
        self.unlocked = true;
        self.crypto_key = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        self.current_screen = CurrentScreen::Main;
    }

    // not sure this is necessary. Consider
    // refactor
    pub fn fetch_items(&mut self, accounts: Vec<Account>) {
        self.accounts = accounts;
    }

    fn decrypt_account(account: &mut Account, master_pass: &[u8]) {
        if !&account.user.is_empty() {
            let decrypted_user =
                decrypt(&account.user, master_pass).expect("failed to decrypt username");

            account.user = decrypted_user;
        }

        if !&account.pass.is_empty() {
            let decrypted_pass =
                decrypt(&account.pass, master_pass).expect("failed to decrypt password");

            account.pass = decrypted_pass;
        } else {
            account.pass = b"Empty pass".to_vec();
        }
    }

    pub fn update_active_account(&mut self, index: usize) {
        self.active_account = Some(self.accounts[index].clone()); // TODO: FIX THIS!
        if let Some(ref mut acc) = self.active_account {
            Self::decrypt_account(acc, self.crypto_key.as_bytes());
        }
    }

    pub fn copy_pass(&mut self) {
        let mut clip = ClipboardContext::new().expect("failed to create ClipboardContext");
        if let Some(acc) = &self.active_account {
            clip.set_contents(acc.clone().get_pass()); // TODO: FIX CLONE
            self.pass_copied = true;
        }
    }

    pub fn clear_clipboard(&mut self) {
        let mut clip = ClipboardContext::new().expect("failed to create ClipboardContext");
        let _ = clip.clear();
        self.pass_copied = false;
    }
}

pub fn list_items() -> String {
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

pub fn parse_items(items: String, crypto_key: String) -> Vec<Account> {
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
            let id = clean_string(current_object[0].split(":").collect::<Vec<&str>>()[1]);
            let name = clean_string(current_object[1].split(":").collect::<Vec<&str>>()[1].trim());

            let mut raw_user =
                clean_string(current_object[2].split(":").collect::<Vec<&str>>()[1].trim());
            let mut raw_pass =
                clean_string(current_object[3].split(":").collect::<Vec<&str>>()[1].trim());

            let mut user: Vec<u8> = vec![];
            if !raw_user.is_empty() {
                user = encrypt(raw_user.as_bytes(), crypto_key.as_bytes())
                    .expect("failed to encrypt username");
                raw_user.zeroize();
            }

            let mut pass: Vec<u8> = vec![];
            if !raw_pass.is_empty() {
                pass = encrypt(raw_pass.as_bytes(), crypto_key.as_bytes())
                    .expect("failed to encrypt password");
                raw_pass.zeroize();
            }

            accounts.push(Account {
                id: id.to_string(),
                name: name.to_string(),
                user,
                pass,
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
