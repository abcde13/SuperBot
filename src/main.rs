
extern crate discord;
extern crate libc;

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
use std::process::Command;
use libc::*;
use std::str;
use std::str::FromStr;
use std::process;
use std::mem;
use std::ptr;
use std::env;

// fails, PR already open #35897 
//const VOICE_CHANNEL: &  str =  "test";
//const USERNAME: &  str = "JtotheC";


fn main() {


    let voice_channel  = "test";
    let username = "JtotheC";

    let mut bot_in_channel = false;
    let mut channel_id: Option<ChannelId>= None;
    let mut server: Option<ServerId>  = None;
    let mut user_in_channel = false;
    let mut user_in_game = false;
    //let mut voice: Option< &mut VoiceConnection> = None;

    /* list of possible address
     *
     * 0x5c45ce
     * 0x5d924b0,
     * 0x201386f4
     * */

    /*let output = Command::new("pgrep")
                          .arg("RocketLeague")
                          .output()
                          .expect("where's the PID?");


    let mut tmp_pid: String = String::from_utf8(output.stdout).unwrap();

    let tmp_pid_len = tmp_pid.len();

    tmp_pid.truncate(tmp_pid_len-1);

    let pid: pid_t = tmp_pid.parse::<i32>().unwrap();

    unsafe{

        let mut addr = 0x20164e84;

        for x in 0..3000{

            addr = addr + 1;
            let mut value: u32 = mem::uninitialized();
            let local_iov = iovec {
            iov_base: &mut value as *mut _ as *mut c_void,
            iov_len: mem::size_of::<u32>(),
            };
            let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: mem::size_of::<u32>(),
            };


            let read = process_vm_readv(pid, &local_iov, 1, &remote_iov, 1, 0);

            println!("addr: {:#x}",addr );
            println!("value: {}",value );
            println!("read: {}",read );

        }

    };*/

    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token")
    ).expect("login failed");



    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::VoiceStateUpdate(server_opt,voice_state)) => {
                println!("Got voice update: {:?},{:?}",server_opt,voice_state.channel_id);
                if(server_opt.is_some() && voice_state.channel_id.is_some()){
                    let server = server_opt;
                    let user = discord.get_member(server_opt.expect("No Server"),voice_state.user_id).unwrap();
                    let channel: Channel = discord.get_channel(voice_state.channel_id.expect("No Channel")).unwrap();
                    match channel {
                        Channel::Public(ref voice) if voice.kind == ChannelType::Voice => {
                            if(user.display_name() == username && voice.name == voice_channel){
                                user_in_channel = !user_in_channel;
                                println!("user_in_channel {}",user_in_channel);
                                println!("user_in_game {}",user_in_game);
                                channel_id = voice_state.channel_id;
                            }
                            if(user_in_game && user_in_channel && !bot_in_channel){
                                let voice = Some(connection.voice(server));
                                match channel_id {
                                    Some(id) => {
                                        println!("Joining");
                                        voice.map(|v| v.connect(id));
                                        bot_in_channel = true;
                                    }
                                    None => println!("Never found channel id")
                                }
                            }
                        }
                        _ => println!("Not a voice channel")
                    }
                } else {
                    user_in_channel = false;
                }
            },
            Ok(Event::PresenceUpdate{presence,server_id,roles}) => {
                let user = discord.get_member(server_id.expect("No Server"),presence.user_id).unwrap();
                let server = server_id;
                println!("Presence changed of: {}",presence.user_id);
                if(presence.game.is_some()){
                    if(user.display_name() == username && presence.game.expect("No game").name == "Rocket League"){
                        user_in_game = !user_in_game;
                        println!("user_in_channel {}",user_in_channel);
                        println!("user_in_game {}",user_in_game);
                    }
                    if(user_in_game && user_in_channel && !bot_in_channel){
                        let voice = Some(connection.voice(server));
                        match channel_id {
                            Some(id) => {
                                println!("Joining");
                                voice.map(|v| v.connect(id));
                                bot_in_channel = true;
                            }
                            None => println!("Never found channel id")
                        }
                    }
                } else {
                    user_in_game = false;
                }
            },
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            },
            Err(err) => println!("Receive error: {:?}", err)
        }

    }

}
