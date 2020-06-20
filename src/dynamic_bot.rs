use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::internal_api::InternalApi;
use crate::internal_api::ApiResponse::{User, Continue, Logout};

impl DynamicBot
{
    pub fn new() -> LoggedOutDBot
    {
        let users = load_user_data("users.yml".to_string());
        let bot = LoggedOutDBot{registered_users: users};
        bot
    }

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
    pub fn login(self, name: String, channel: String, token: String) -> DynamicBot
    {
        let api = InternalApi::new(name, channel, token);
        let bot = DynamicBot{registered_users: self.registered_users, api: api};
        bot
    }
}

pub struct DynamicBot
{
    registered_users: HashMap<String, String>,
    api: InternalApi,
}

pub struct LoggedOutDBot
{
    registered_users: HashMap<String, String>,
}

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

#[derive(Serialize, Deserialize)]
struct Users
{
    users: HashMap<String, String>,
}

fn load_user_data(file_path: String) -> HashMap<String, String> 
{
    let yaml_string: String= fs::read_to_string(file_path).expect("Couldn't open file");
    let yaml: Users = serde_yaml::from_str(&yaml_string).unwrap();
    yaml.users
}

