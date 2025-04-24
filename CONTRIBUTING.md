> [!NOTE]
> This was developed on windows so you're more on your own if on mac or linux

### Prerequisites
- Git
- Rustc
- [msvc build tools](https://visualstudio.microsoft.com/downloads/?q=build+tools)
    - You can also use this [gist](https://gist.github.com/mmozeiko/7f3162ec2988e81e56d5c4e22cde9977) if you just need
      the build tools for rust
    - Gnu toolchain also works but isnt recommended
- sqlite3
- A Discord bot

### Setting Up
- Clone the repository
```sh
git clone https://github.com/Not-a-cowfr/Cow-bot.git
```
- Fill out required environment variables
    - Create a copy of [.env.example](.env.example) and rename it to `.env`
    ```sh
    cp .env.example .env
    ```
    - Create a [discord bot](https://discord.com/developers/applications) and copy its private token, add it to the `.env`
- Run the bot
```sh
cargo run
```

### How to add a command
- Add a file ending with `_command` in `src/commands/` or a subdirectory of that
- Create a function in that file with the same name as the file, excluding the `_command`
- Make sure your command include poise macro to define what kind of command it is, and takes a context param

for example: this would be `src/commands/nested_dir/nested_dir/example_command.rs`
```rust
use crate::{Context, Error};

/// slash command descriptions are made like this with 3 /
#[poise::command(slash_command)]
pub async fn example(
	ctx: Context<'_>,
) -> Result<(), Error> {
    // do whatever you want, I recommend checking out the poise and serenity docs or looking at some of the other existing commands
    Ok(())
}
```
This will now be automatically generated as a command upon running thanks to [build.rs](build.rs)
