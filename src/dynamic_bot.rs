use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::internal_api::InternalApi;
use crate::internal_api::ApiResponse::{User, Continue, Logout};

impl DynamicBot
{
    //Constructor returns LoggedOutDBot with registered user list
    pub fn new() -> LoggedOutDBot
    {
        let users = load_user_data("users.yml".to_string());
        let bot = LoggedOutDBot{registered_users: users};
        bot
    }

    //Listens for users to login from api and returns music
    pub fn listen_respond_logout(self) -> LoggedOutDBot
    {
        loop
        {
            match self.api.recieve_response()
            {
                User(name) => {
                    let music = self.get_music(name);
                    self.api.send_music(music);
                },
                Continue() => continue,
                Logout() => break
            }
        }
        let bot = LoggedOutDBot{registered_users: self.registered_users};
        bot
    }
}

impl LoggedOutDBot
{
    //Login function returns usable dbot 
    pub fn login(self, name: String, channel: String, token: String) -> DynamicBot
    {
        let api = InternalApi::new(name, channel, token);
        let bot = DynamicBot{registered_users: self.registered_users, api: api};
        bot
    }
}

//DynamicBot stores HashMap of users and their music and api
pub struct DynamicBot
{
    registered_users: HashMap<String, String>,
    api: InternalApi,
}

//LoggedOutDBot only stores HashMap of users.
pub struct LoggedOutDBot
{
    registered_users: HashMap<String, String>,
}

//Private function accesses internal hashmap
impl DynamicBot
{
    fn get_music(&self, user: String) -> String
    {
        match self.registered_users.get(&user)
        {
            Some(music) => return music.to_string(),
            _ => return "A New Challenger".to_string(),
        }
    }
}

//Users struct to load data from yml file
#[derive(Serialize, Deserialize)]
struct Users
{
    users: HashMap<String, String>,
}

//Loads data from yaml file
fn load_user_data(file_path: String) -> HashMap<String, String> 
{
    let yaml_string: String= fs::read_to_string(file_path).expect("Couldn't open file");
    let yaml: Users = serde_yaml::from_str(&yaml_string).unwrap();
    yaml.users
}

