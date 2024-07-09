use std::collections::HashMap;

pub struct App {
    pub items: Vec<Account>,
    pub selected: usize,
}

// TODO: fix permissions
#[derive(Clone)] // TODO: FIX THIS!
pub struct Account {
    pub id: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

impl Account {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_user(&self) -> String {
        self.user.clone()
    }

    fn get_pass(self) -> String {
        self.pass
    }
}

impl App {
    pub fn new() -> App {
        App {
            items: vec![],
            selected: 0,
        }
    }

    pub fn fetch_items(&mut self, items: Vec<Account>) {
        self.items = items;
    }
}
