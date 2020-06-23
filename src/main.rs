mod dynamic_bot;
mod api;

use dynamic_bot::DynamicBot;

fn main() 
{
    let bot = DynamicBot::new("config.yml".to_string());
    let bot = bot.login();
    bot.listen_respond_logout().unwrap();
}
