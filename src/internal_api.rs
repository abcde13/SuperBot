use std::thread;
use std::sync::mpsc;

type Tx = mpsc::Sender<ApiResponse>;
type Rx = mpsc::Receiver<ApiResponse>;

type MTx = mpsc::Sender<String>;
type MRx = mpsc::Receiver<String>;

pub struct InternalApi
{
    token: String,
    api_rx: Rx,
    music_tx: MTx,
    close_tx: Tx,
}

pub enum ApiResponse
{
    User(String),
    Logout(),
}

impl InternalApi
{
    pub fn new(token: String) -> InternalApi
    {
        let local_token = token.clone();
        let (api_tx, api_rx) = mpsc::channel::<ApiResponse>();
        let (music_tx, music_rx) = mpsc::channel::<String>();
        let (close_tx, close_rx) = mpsc::channel::<ApiResponse>();
        thread::spawn(move || spawn_api(token, api_tx, music_rx, close_rx));
        let api = InternalApi{token: local_token, 
            api_rx: api_rx, music_tx: music_tx, close_tx: close_tx};
        api
    }

    pub fn recieve_response(&self) -> ApiResponse
    {
        match self.api_rx.recv().unwrap()
        {
            ApiResponse::User(name) =>
            {
                return ApiResponse::User(name.to_string());
            },
            ApiResponse::Logout() => return ApiResponse::Logout()
        }
    }

    pub fn send_music(&self, music: String)
    {
        self.music_tx.send(music);
    }

    pub fn close(self) -> String
    {
        self.close_tx.send(ApiResponse::Logout());
        self.token.clone()
    }
}

//Spawns external api and waits for close signal
fn spawn_api(token: String, user_tx: Tx, music_rx: MRx, close_rx: Rx)
{
    // Configure the client with your Discord bot token in the environment.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<ApiSender>(Arc::new(Mutex::new(user_tx)));
        data.insert::<MusicReceiver>(Arc::new(Mutex::new(music_rx)));
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix("~"))
        .group(&GENERAL_GROUP));
    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
    let signal = close_rx.recv().unwrap();
}

//Api code at the bottom to allow for easy removal
use std::{env, sync::Arc};

use serenity::{
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready, id::ChannelId, misc::Mentionable},
    prelude::*,
    voice::AudioReceiver,
    Result as SerenityResult,
};

//Definitions for data to send to Rust api
struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct ApiSender;

impl TypeMapKey for ApiSender {
    type Value = Arc<Mutex<Tx>>;
}

struct MusicReceiver;

impl TypeMapKey for  MusicReceiver{
    type Value = Arc<Mutex<MRx>>;
}


struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct Receiver;

impl AudioReceiver for Receiver {
    fn client_connect(&mut self, _ssrc: u32, _user_id: u64) {
        println!("{} is connected!", _user_id);
    }
}

#[group]
#[commands(join, leave)]
struct General;

#[command]
fn join(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let connect_to = match args.single::<u64>() {
        Ok(id) => ChannelId(id),
        Err(_) => {
            check_msg(msg.reply(&ctx, "Requires a valid voice channel ID be given"));

            return Ok(());
        },
    };

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.join(guild_id, connect_to) {
        handler.listen(Some(Box::new(Receiver{})));
        check_msg(msg.channel_id.say(&ctx.http, &format!(";;join")));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));
    }

    Ok(())
}

#[command]
fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported"));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say(&ctx.http,";;leave"));
    } else {
        check_msg(msg.reply(&ctx, "Not in a voice channel"));
    }
    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
