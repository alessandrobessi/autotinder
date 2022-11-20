mod requests;
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use colored::Colorize;
use requests::{get_recommendations, like};
use serde_json::Value;
use tokio::time::{sleep, Duration};

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
    let fifteen_seconds: u64 = 15000;
    let token: String;
    let args: Arguments = Arguments::parse();
    token = args.token;
    loop {
        let response: Result<String, reqwest::Error> = get_recommendations(&token).await;
        match response {
            Ok(value) => {
                let v: Value = serde_json::from_str(&value).expect("JSON was not well-formatted");
                for i in v["data"]["results"].as_array().unwrap() {
                    let id: &str = i["user"]["_id"].as_str().unwrap();
                    let name: &str = i["user"]["name"].as_str().unwrap();
                    let s_number: u64 = i["s_number"].as_u64().unwrap();
                    let photo_id: &str = i["user"]["photos"]
                        .as_array()
                        .unwrap()
                        .first()
                        .unwrap()
                        .as_object()
                        .unwrap()["id"]
                        .as_str()
                        .unwrap();

                    // like
                    let res: Result<String, reqwest::Error> =
                        like(&token, &id, &photo_id, s_number).await;
                    match res {
                        Ok(value) => {
                            let v: Value =
                                serde_json::from_str(&value).expect("JSON was not well-formatted");

                            if v["likes_remaining"].as_u64().unwrap() == 0 {
                                let deadline: i64 = v["rate_limited_until"].as_i64().unwrap();
                                let datetime: Option<NaiveDateTime> =
                                    NaiveDateTime::from_timestamp_millis(deadline);
                                let now: DateTime<Utc> = Utc::now();
                                let pause_in_millis: i64 =
                                    datetime.unwrap().timestamp() - now.timestamp();
                                let sleep_time: Duration = Duration::from_millis(
                                    u64::try_from(pause_in_millis).unwrap() * 1000,
                                );
                                let pause_in_hours: f64 = (pause_in_millis as f64) / (60.0 * 60.0);

                                println!(
                                    "{}",
                                    format!(
                                        "Tinder put you on hold until {} UTC. Sleeping for ~{:.0} hours...",
                                        datetime.unwrap(),
                                        pause_in_hours
                                    )
                                        .bold()
                                        .red()
                                );
                                sleep(sleep_time).await;
                            }

                            if v["status"].as_u64().unwrap() == 200 {
                                print!("You swiped right on {}!", name);
                            }
                            if v["match"].as_bool().unwrap() == true {
                                print!(" | It's a match!");
                            } else {
                                print!(" | Not yet a match...");
                            }
                            println!(
                                " | {} likes remaining",
                                v["likes_remaining"].as_u64().unwrap()
                            );
                        }

                        Err(e) => {
                            println!("REQUEST ERROR\n{e:?}");
                        }
                    }
                    sleep(Duration::from_millis(fifteen_seconds)).await;
                }
            }
            Err(e) => println!("REQUEST ERROR\n{e:?}"),
        }
    }
}
