> [!NOTE]
> This was developed on windows so you're more on your own if on mac or linux

# Prerequisites
- Git
- Rustc
- [msvc build tools](https://visualstudio.microsoft.com/downloads/?q=build+tools)
    - You can also use this [gist](https://gist.github.com/mmozeiko/7f3162ec2988e81e56d5c4e22cde9977) if you just need
      the build tools for rust
    - Gnu toolchain also works but isnt recommended
- sqlite3
- A Discord bot

# Setting Up
- Clone the repository
```sh
git clone https://github.com/cow-discord-bot/bot.git
```
- Fill out required environment variables
    - Create a copy of [.env.example](.env.example) and rename it to `.env`
    ```sh
    cp .env.example .env
    ```
    - Create a [discord bot](https://discord.com/developers/applications) and copy its private token, add it to the `.env`
- Run the bot
```sh
./run
```

<details><summary><h1>If you want to run in release, using a domain for the api</h1></summary>

### Prerequisites
- Previous prerequisites
- [nginx](https://nginx.org/)

### Steps
1. Create an A record pointing to the ip you're hosting the api on
2. In your nginx conf dir add a file `conf.d/<your chosen domain name>.conf` and add this:
```conf
server {
    listen 80;
    server_name <your domain>;

    location / {
        proxy_pass http://localhost:3000; # change the port if you chose something else for the API_PORT env var
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```
3. Also in your nginx conf dir add this to your `nginx.conf`
```conf
include conf.d/*.conf;
```
4. In the root project directory run
```sh
./run release
```
The build script relies on your nginx directory too look something like this
```
nginx dir
  ├──nginx.exe
  └──conf
    ├──nginx.conf
    └──conf.d
      └──<your filename from step 2>
```
It also relies on your nginx path to be in your PATH environment variable

## If you want to use https with nginx

### Prerequisites
- Previous prerequisites
- [win acme](https://github.com/win-acme/win-acme/releases/tag/v2.2.9.1701)

### Steps
1. Generate certificate
```sh
wacs --source manual --host <your domain> --validation filesystem --webroot "<nginx dir>/html" --store pemfiles --pemfilespath "<nginx dir>/certs"
```
2. Accept the terms they give you and enter your email for notifications, I don't remmeber if the email is optional
3. Update your conf file from step 2 of the previous set of instructions
```conf
server {
    listen 80;
    server_name <your domain name>;
    location /.well-known/acme-challenge/ {
        root <nginx dir>/html;
        allow all;
    }
    location / {
        return 301 https://$host$request_uri;
    }
}

server {
    listen 443 ssl;
    server_name <your domain name>;
    ssl_certificate <nginx dir>/certs/<your domain name>-chain.pem;
    ssl_certificate_key <nginx dir>/certs/<your domain name>-key.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    location / {
        proxy_pass http://localhost:3000; # change the port if you chose something else for the API_PORT env var
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

</details>

# How to add a command
- Add a file ending in `crates/bot/src/commands/` or a subdirectory of that
- Create a function in that file with the same name as the file
- Make sure your command include poise macro to define what kind of command it is, and takes a context param

for example: this would be `crates/bot/src/commands/nested_dir/nested_dir/example.rs`
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
This will now be automatically generated as a command upon running thanks to [build.rs](crates/bot/src/build.rs)

# How to add an api endpoint
- Create a file in `src/endpoints` thats nested matching its url endpoint
- If a part of that url is a paremeter, show that in the directory by anming the coresponding dir name with a $ in the beginning

for example: this would be `./crates/api/src/endpoints/nested_dir/$param_nested_dir/example/whatever_you_want_to_name_this.rs`
```rust
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;

// function name has to be same as file name
pub fn whatever_you_want_to_name_this() -> Router { Router::new().route("nested_dir/{param_nested_dir}/example", post(handle_request)) }

async fn handle_request() -> impl IntoResponse {
	(StatusCode::OK, "this endpoint is an example!".to_string())
}
```
This will now be automatically generated as an endpoint upon running thanks to [build.rs](crates/api/src/build.rs)

Currently the build script for the endpoints has 2 bugs
1. It wont work if there are no endpoints
- This isn't really an issue, there will never be 0 endpoints but it's just good to note
2. You can't put an endpoint file directly in the `src/endpoints/` dir, it needs to be nested
- Again, won't really affect anything due to how endpoints are supposed to be made anyway

Maybe in the future the route will be fixed automatically based on the defined endpoint or the other way around
If anyone cares i whipped up this regex in a few minutes for finding the router function
`pub fn (?<function_name>\w+)\(\) -> Router {(?:\n\t)? ?Router::new\(\).route\("(?<url_endpoint>.*)", (?<request_type>\w+)\((?<request_handler>\w+)\)\)\n? ?}`
