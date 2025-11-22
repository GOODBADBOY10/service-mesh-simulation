use std::{collections::HashMap, sync::{Mutex}};
use crate::register::User;

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>,
}