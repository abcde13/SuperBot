use std::thread;
use std::sync::mpsc;

use super::serenity::{spawn_api, Rx, MTx};

pub enum ApiMessage
{
    User(String),
    Logout(),
    LogoutWithToken(String),
}

impl DiscordApi
{
    pub fn new(token: String) -> DiscordApi
    {
        let local_token = token.clone();
        let (api_tx, api_rx) = mpsc::channel::<ApiMessage>();
        let (music_tx, music_rx) = mpsc::channel::<String>();
        thread::spawn(move || spawn_api(token, api_tx, music_rx));
        let api = DiscordApi{token: local_token, 
            api_rx: api_rx, music_tx: music_tx};
        api
    }

    pub fn recieve_response(&self) -> ApiMessage
    {
        match self.api_rx.recv().unwrap()
        {
            ApiMessage::User(name) =>
            {
                return ApiMessage::User(name.to_string());
            },
            ApiMessage::Logout() => return ApiMessage::LogoutWithToken(self.token.clone()),
            ApiMessage::LogoutWithToken(token) => return ApiMessage::LogoutWithToken(token),
        }
    }

    pub fn send_music(&self, music: String)
    {
        self.music_tx.send(music).expect("Unable to send music through api");
    }
}

pub struct DiscordApi
{
    token: String,
    api_rx: Rx,
    music_tx: MTx,
}

