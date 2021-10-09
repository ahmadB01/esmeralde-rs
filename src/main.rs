use chrono::{Datelike, Local};
use regex::Regex;
use structopt::StructOpt;

use std::{env, error::Error, fs::File, io::BufReader, path::PathBuf};

use serenity::{
    async_trait,
    framework::standard::{macros::*, Args, CommandResult, StandardFramework},
    model::{channel::Message, error, gateway::Ready},
    prelude::*,
};

struct Groups;

impl TypeMapKey for Groups {
    type Value = serde_json::Value;
}

#[group]
#[commands(edt)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, msg: Ready) {
        println!("Bot {} is connected!", msg.user.name);
    }
}

fn read_groups(arg_groups: PathBuf) -> Result<serde_json::Value, Box<dyn Error>> {
    let file = File::open(arg_groups)?;
    let reader = BufReader::new(file);
    let groups = serde_json::from_reader(reader)?;
    Ok(groups)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "esmeralde-rs", about = "Run Esmeralde Discord bot")]
struct Opt {
    /// Discord bot token.
    /// If not provided, env variable "DISCORD_TOKEN" is used instead.
    #[structopt(short, long)]
    token: Option<String>,

    /// Absolute path of groups config file (JSON).
    groups_file: PathBuf,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let fw = StandardFramework::new()
        .configure(|c| c.prefix("/").delimiters(vec![' ', '-', '_']))
        .group(&GENERAL_GROUP);

    let groups = read_groups(opt.groups_file).expect("Unable to load groups config file");
    let token = opt
        .token
        .or(env::var("DISCORD_TOKEN").ok())
        .expect("Token not provided");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(fw)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Groups>(groups);
    }

    if let Err(why) = client.start().await {
        println!("error while starting client: {:?}", why);
    }
}

fn get_link(mut id: String) -> String {
    id.retain(|c| c != '"');
    let now = Local::now();
    let year = now.year();
    let week = now.iso_week().week();

    format!("https://edt.iut-orsay.fr/vue_invite_horizontale.php?current_year={}&current_week={}&groupes_multi%5B%5D={}&lar=1920&hau=1200", year, week, id)
}

#[command]
#[aliases("emploi", "agenda", "calendar")]
#[description("Renvoie l'emploi du temps")]
// TODO: add week selection arg
#[max_args(2)]
async fn edt(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let groups = data.get::<Groups>().expect("Expected Groups in TypeMap.");

    let mut args = args.iter::<String>();

    let mut idx = String::new();
    if let Some(first) = args.next() {
        // Infaillible
        let first = first.unwrap().to_lowercase();
        if first == "tp" {
            if let Some(second) = args.next() {
                // Infaillible
                let second = second.unwrap();
                idx = format!("TP{}", second);
            } else {
                idx = "wtf".to_string();
            }
        } else if first.contains("tp") {
            idx = first;
        } else {
            idx = format!("TP{}", first);
        }
    } else {
        let member = msg.member.as_ref().ok_or(error::Error::MemberNotFound)?;

        for roleid in &member.roles {
            let mut role_name = roleid
                .to_role_cached(&ctx.cache)
                .await
                .ok_or(error::Error::RoleNotFound)?
                .name
                .to_lowercase();

            let reg = Regex::new(r"(tp)?[-_ ]*[0-9][a-z]").unwrap();
            if reg.is_match(role_name.as_str()) {
                role_name.retain(|c| ![' ', '_', '-'].contains(&c));
                idx = if role_name.contains("tp") {
                    role_name
                } else {
                    format!("tp{}", role_name)
                };
            }
        }
    }

    let gid = groups.get(idx.to_uppercase());
    match gid {
        Some(id) => {
            msg.reply(&ctx.http, get_link(id.to_string())).await?;
        }
        None => {
            msg.reply(&ctx.http, "Unknown group :(").await?;
        }
    }

    Ok(())
}
