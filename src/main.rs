mod dynamic_bot;
mod internal_api;

use dynamic_bot::DynamicBot;

fn main() 
{
    let bot = DynamicBot::new("config.yml".to_string());
    let bot = bot.login();
    bot.listen_respond();
}
