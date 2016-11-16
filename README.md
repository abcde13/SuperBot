
## SuperBot

#### The Rocket League Discord Bot

##### What does it do?

This bot's primary purpose is to play some kind of sound when a goal is scored
for your team (or maybe against your team) in [Rocket League](https://www.rocketleaguegame.com/).

##### So why is it called "Super" Bot?

That's because my twin brother and I had this amazing idea that every time
we scored a goal, we could get [Franky from One Piece saying his signature 
"SUUUUUUUUPERRRRRRRRRRRRRRR"](https://youtu.be/IvlWP4lQ6m8). Funny, right? Thanks, we know.

Another inspiration came from the [Goalie Discord Bot](https://www.reddit.com/r/RocketLeagueMods/comments/5c5ia4/goalie_the_discord_bot_that_plays_music_when_you/)
that redditor /u/garretjones331 made. It, however, is Windows only. As a Rocket League player
who now plays exclusively on Linux, I wanted the bot. And as a hacker, I set out to build my own.

##### Enough of that. How do I use this bot?


Easier said that done.

  1. Clone this repo to wherever you want the bot to run from.
  2. Follow [this link's steps](https://github.com/reactiflux/discord-irc/wiki/Creating-a-discord-bot-&-getting-a-token)
to get your app setup with.
  3. Create an environment variable name "DISCORD_TOKEN" in your ~/.bashrc, ~/.bash_profile, ~/.zshrc,
  or whatever fancy file gets called on shell initialization. Just check that before you run the bot,
  `env | grep DISCORD` returns the `DISCORD_TOKEN=yourtokenhere`. 
  4. Now comes the tough part. Currently, there is no way to pipoint the exact address Rocket League uses to store the
  value of how many goals your team has scored due to the nature of the Linux kernel (extra security measures compared
  to Windows, yay for most, nay for us right now). You'll need to do the following to (cross your fingers) get this working:

  &nbsp;&nbsp;&nbsp;&nbsp;1. Install a tool called [scanmem](https://github.com/scanmem/scanmem) or it's non-gui counterpart, GameConqueror.
      GameConqueror is supposed to be the CheatEngine for Linux. We can do `sudo-apt-get install scanmem gameconqueror` or
      follow the instructions in the link for scanmem.

  &nbsp;&nbsp;&nbsp;&nbsp;2. Run Rocket League, and attach scanmem to the process pid of RocketLeague (can be found by using `pgrep RocketLeague`),
      by typing (in a new terminal tab) `scanmem` followed by `pid YOUR PID`. If using GameConqueror, use the little "Select
      a process" tool to do that.

  &nbsp;&nbsp;&nbsp;&nbsp;3. Now, in Rocket League, start an exhibition match and score a goal for your team. Now, go back to scanmem or GameConqueror
      and type in `1` and hit enter (or scan). The first search will take a while, because it has to search through most of the memory
      space for this number. You'll end up with a large number of hits (at least a million, I thinkg).

  &nbsp;&nbsp;&nbsp;&nbsp;4. Repeat c) until you have between 5 to 40 matches. Every time you score a goal, type in the number that matches the goals your team has. 

  &nbsp;&nbsp;&nbsp;&nbsp;5. Pick an address that is near the top of the list (smaller address). Don't pick the top, just . . . near it. Copy that address.

  5. Back here. Go to the folder where you cloned this bot. Now, you have two options (IN WORKS). You can either go into src/main.rs, 
  find where the `START_ADDR` is specified, and paste your address here, or you can run `cargo run YOUR_USERNAME YOUR_CHANNEL YOUR_ADDR`, passing it in as a 
  command line argument (only first one works as of now).
  6. Run the bot (making sure you have cargo and rust installed) with `cargo run YOUR_USERNAME YOUR CHANNEL`. `YOUR_USERNAME` = your username on Discord, and
  `YOUR_CHANNEL` = the name of the voice channel you want the bot to join. Now the bot will only join the Discord voice channel once you join the channel *and*
  when you start playing Rocket League. **Make sure you enable Discord's ability to track what game you are currently playing**.
  7. **TODO** Store the mp3 files of the sounds you want SuperBot to play in the folder named "music".


That was probably more than you expected. But it's a one time deal. Have fun.



