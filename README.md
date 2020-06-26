
## DynamicEntryBot

This is small personal project to experiment with ideas when writing Rust code.
I do not plan to support it as an actual discord bot. The blog post 
describing this in more detail is [here](https://mitigatingfailure.com/Design%20Patterns%20in%20Rust.html).

### What does it do?

This bot is supposed to play music when a user enters a discord channel.

### Usage  
As this is research project the usability is weak. If you really want to
you can try these instructions but they're not particularly robust.

You need to have youtube-dl working from the command line. I use linux so
this is easy with the package manager but you'll need to figure out how to 
do so on other platforms. This is the [project](https://github.com/ytdl-org/youtube-dl).
youtube.

Use [this](https://discordpy.readthedocs.io/en/latest/discord.html) tutorial to set up the discord bot.

Download the project and create a yaml file called config.yml in the top level directory. The file should looke like this

token: sdkfjdkjfdkfjd\
users:\
>  12323232323:"youtube link"
  
The bot needs a user id, the user name won't work(like I said, poor usability)

You can then start the app with cargo run and should see it in discord

The command "~join 34343434" will join your channel, where the number also corresponds to 
the voice channel's id, not it's name. Then, after a small delay for downloading; the bot 
will play the music file.
