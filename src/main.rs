
extern crate discord; extern crate libc; #[macro_use]
extern crate log;


use discord::Discord;
use discord::State;
use discord::Connection;
use discord::voice::VoiceConnection;
use discord::model::Event;
use discord::model::CurrentUser;
use discord::model::ChannelType;
use discord::model::Channel;
use discord::model::User;
use discord::model::ChannelId;
use discord::model::ServerId;
use discord::model::VoiceState;
use discord::model::Presence;
use discord::model::RoleId;
use discord::model::PublicChannel;
use std::process::Command;
use libc::*;
use std::str;
use std::str::FromStr;
use std::process;
use std::mem;
use std::ptr;
use std::env;
use std::io;
use std::io::Stdout;
use std::io::Write;
use std::collections::HashMap;
use std::time::Duration;

//Environment variables to look up
static DISCORD_NAME: &'static str = "DISCORD_NAME";
static DISCORD_CHANNEL: &'static str = "DISCORD_CHANNEL";
static DISCORD_TOKEN: &'static str = "DISCORD_TOKEN";

//Discord Info struct
struct DiscordInfo<'a> {
    username: &'a str,
    channel: &'a str,
}

//State struct
struct DiscordState<'a> {
    bot_in_channel: &'a mut bool,
    channel_id: &'a mut Option<ChannelId>,
    user_in_channel: &'a mut bool,
    user_in_game: &'a mut bool,
}

            

fn main() {

    //Setting token from environment variable
    let YOUR_TOKEN: &str = &env::var(DISCORD_TOKEN).unwrap();

    //Print statements for debugging
    println!("{:?}", YOUR_TOKEN);

    //Declarations for address search
    let mut pid: pid_t = -1;
    let mut possible_addrs: &mut HashMap<i32,i32> = &mut HashMap::new();

    //Start search hotfix variable
    let mut start_search = false;

    let discord = log_into_discord(YOUR_TOKEN);
    let (mut connection, _) = discord.connect().expect("connect failed");

    let mut discord_closure  = || dispatch_on_event(&discord, &mut connection);

    let mut rl_score_closure = || rl_score_check(0x2cc42d5c, possible_addrs, pid);

    println!("Ready.");
    loop {
        println!("looping {}",start_search);

        if !start_search { 
            discord_closure();
        } else {
            rl_score_closure();
        }


    }


}

/// Fetch pid of Rocket League
fn fetch_rl_pid(pid: pid_t) -> pid_t{

    let output = Command::new("pgrep")
        .arg("RocketLeague")
        .output()
        .expect("where'sthe PID?");
     let mut tmp_pid: String = String::from_utf8(output.stdout).unwrap();
     let tmp_pid_len = tmp_pid.len();
     tmp_pid.truncate(tmp_pid_len-1);
     tmp_pid.parse::<i32>().unwrap()
}
 
/// Function takes in bot token and logs into discord; returning a session object.
fn log_into_discord(token: &str) -> Discord {
    Discord::from_bot_token(token).expect("login failed")
}

/// Function listens for events on the voice channel and runs functions in response.
fn dispatch_on_event(discord: &Discord, connection: &mut Connection) {

    //Initialzing username and channel from environment variables
    let username: &str  = &env::var(DISCORD_NAME).unwrap();
    let channel_name: &str = &env::var(DISCORD_CHANNEL).unwrap();
    let discord_info = DiscordInfo { username: username, channel: channel_name};

    //Print statements for debugging
    println!("{:?}", username);
    println!("{:?}", channel_name);

    //Initializing state variables
    let mut discord_state = DiscordState { bot_in_channel: &mut false, channel_id: &mut None, user_in_channel: &mut false, user_in_game: &mut false};

    match connection.recv_event() {
        
        Ok(Event::VoiceStateUpdate(server_opt, voice_state)) => voice_channel_update_event(discord, connection, &server_opt, &voice_state, &discord_info, &mut discord_state),

        Ok(Event::PresenceUpdate{presence, server_id, roles}) => game_state_update_event(discord, connection, presence, &server_id, &roles, &discord_info, &mut discord_state),

        Ok(_) => {}
        
        Err(discord::Error::Closed(code, body)) => {
            println!("Gateway closed on us with code {:?}: {}", code, body);
            process::exit(1);
        },

        Err(err) => println!("Receive error: {:?}", err)
    }
}

/// Event runs when an occurence happens on the voice channel.
fn voice_channel_update_event(discord: &Discord, connection: &mut Connection, server_opt: &Option<ServerId>, voice_state: &VoiceState, info: &DiscordInfo, state: &mut DiscordState) {
    println!("Got voice update: {:?},{:?}",server_opt,voice_state.channel_id);

    if server_opt.is_some() && voice_state.channel_id.is_some() {

        let server = server_opt;
        let user = discord.get_member(server_opt.expect("No Server"),voice_state.user_id).unwrap();
        let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();

        match channel {
            Channel::Public(ref voice) if voice.kind == ChannelType::Voice => channel_is_voice(discord, connection, server, voice, voice_state, info, state),

            _ => println!("Not a voice channel"),
        }
    } else {
        *state.user_in_channel = false;
    }
}

/// Event dispatch for when state of game changes.
fn game_state_update_event(discord: &Discord, connection: &mut Connection, presence: Presence, server_id: &Option<ServerId>, roles: &Option<Vec<RoleId>>, info: &DiscordInfo, state: &mut DiscordState) {
    let user = discord.get_member(server_id.expect("No Server"), presence.user_id).unwrap();
    let server = server_id;

    println!("Presence changed of: {}", presence.user_id);

    if presence.game.is_some() {

        let name_match_and_playing_game = user.display_name() == info.username && presence.game.expect("No game").name == "Rocket League";

        if  name_match_and_playing_game {

            *state.user_in_game = !*state.user_in_game;
            println!("user_in_channel {}", *state.user_in_channel);
            println!("user_in_game {}", *state.user_in_game);

        }

        check_state_and_join_channel(connection, server, state);

    } else {

        *state.user_in_game = false;

    }
}

/// Update state channel id if voice channel.
fn channel_is_voice(discord: &Discord, connection: &mut Connection, server: &Option<ServerId>, voice: &PublicChannel, voice_state: &VoiceState, info: &DiscordInfo, state: &mut DiscordState) {
    let user = discord.get_member(server.expect("No Server"),voice_state.user_id).unwrap();
    let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();

    let name_and_channel_match = user.display_name() == info.username && voice.name == info.channel;

    if  name_and_channel_match {
        *state.user_in_channel = !*state.user_in_channel;
        println!("user_in_channel {}", state.user_in_channel);
        println!("user_in_game {}", state.user_in_game);
        *state.channel_id = voice_state.channel_id;
    }

    check_state_and_join_channel(connection, server, state);
}

/// Verifies desired state and has bot take action.
fn check_state_and_join_channel(connection: &mut Connection, server: &Option<ServerId>, state: &mut DiscordState) {
    let in_game_in_channel_bot_not_in_channel = *state.user_in_game && *state.user_in_channel && !*state.bot_in_channel;

    if in_game_in_channel_bot_not_in_channel {

        let voice = Some(connection.voice(*server));
        match *state.channel_id {

            Some(id) => {
                println!("Joining");
                voice.map(|v| v.connect(id));
                *state.bot_in_channel = true;
            }

            None => println!("Never found channel id")
        }
    }
}

/// Loop to check score in Rocket League
fn rl_score_check(start_addr: i32, addrs_map: & mut HashMap<i32,i32>, mut pid: pid_t) {

    if(pid == -1){
        pid = fetch_rl_pid(pid);
    }

    unsafe{

        // Start address for guessing
        let mut addr = start_addr;

        let mut count = 0u32;

        // Loop to check each address
        for x in 0..100000 {

            addr = addr + 1;
            let mut value:i32 = mem::uninitialized();
            let local_iov = iovec {
            iov_base: &mut value as *mut _ as *mut c_void,
            iov_len: mem::size_of::<i32>(),
            };
            let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: mem::size_of::<i32>(),
            };

            let read = process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0);

            let  val = *addrs_map.entry(addr).or_insert(value);

            if val != i32::max_value() && val+1 == value{
                //println!("We have a goal? {}",addr);
                *addrs_map.entry(addr).or_insert(value);
                count = count + 1;
            } 

            // Comment back in when debugging. Pipe to file for better viewing
             
            //println!("addr: {:#x}",addr );
            //println!("value: {}",value );
            //println!("read: {}",read );
            

        } 
        //io::stdout().flush().unwrap();
        println!("addr: {:#x}",addr );
        println!("count: {}",count);

    };

}
