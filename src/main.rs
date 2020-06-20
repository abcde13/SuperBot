mod dynamic_bot;
mod internal_api;

use dynamic_bot::DynamicBot;

fn main() 
{
    let bot = DynamicBot::new();
    let bot = bot.login("neji49".to_string(), "test".to_string(), "".to_string());
    bot.listen_respond_logout();
}
