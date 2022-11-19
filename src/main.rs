mod requests;
use clap::Parser;
// use colored::Colorize;
use requests::{ get_recommendations, like };
use serde_json::Value;
use tokio::time::{ sleep, Duration };
use chrono::{ NaiveDateTime };

#[derive(Parser)]
#[command(name = "autotinder")]
#[command(author = "Alessandro Bessi <bessimaestro@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "autotinder swipes right for you <3", long_about = None)]
struct Arguments {
    /// X-auth-token
    token: String,
}

#[tokio::main]
async fn main() {
    let five_seconds: u64 = 5000;
    let token;
    let args: Arguments = Arguments::parse();
    token = args.token;
    loop {
        let response: Result<String, reqwest::Error> = get_recommendations(&token).await;
        match response {
            Ok(value) => {
                let v: Value = serde_json::from_str(&value).expect("JSON was not well-formatted");
                for i in v["data"]["results"].as_array().unwrap() {
                    let id = i["user"]["_id"].as_str().unwrap();
                    let name = i["user"]["name"].as_str().unwrap();
                    let s_number = i["s_number"].as_u64().unwrap();
                    let photo_id = i["user"]["photos"]
                        .as_array()
                        .unwrap()
                        .first()
                        .unwrap()
                        .as_object()
                        .unwrap()["id"]
                        .as_str()
                        .unwrap();

                    // like
                    let res: Result<String, reqwest::Error> = like(
                        &token,
                        &id,
                        &photo_id,
                        s_number
                    ).await;
                    match res {
                        Ok(value) => {
                            let v: Value = serde_json
                                ::from_str(&value)
                                .expect("JSON was not well-formatted");

                            if v["likes_remaining"].as_u64().unwrap() == 0 {
                                let deadline = v["rate_limited_until"].as_i64().unwrap();
                                let datetime = NaiveDateTime::from_timestamp_opt(
                                    deadline / 1000,
                                    0
                                );
                                let twelve_hours = Duration::from_millis(1000 * 60 * 60 * 12);
                                println!(
                                    "Tinder put you on hold until {} UTC. Sleeping for 12 hours...",
                                    datetime.unwrap()
                                );
                                sleep(twelve_hours).await;
                            }

                            if v["status"].as_u64().unwrap() == 200 {
                                print!("You swiped right on {}!", name);
                            }
                            if v["match"].as_bool().unwrap() == true {
                                print!("| It's a match!");
                            } else {
                                print!("| Not yet a match...");
                            }
                            println!(
                                "| {} likes remaining",
                                v["likes_remaining"].as_u64().unwrap()
                            );
                        }

                        Err(e) => {
                            println!("REQUEST ERROR\n{e:?}");
                        }
                    }
                    sleep(Duration::from_millis(five_seconds)).await;
                }
            }
            Err(e) => println!("REQUEST ERROR\n{e:?}"),
        }
    }
}