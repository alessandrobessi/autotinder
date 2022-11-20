use std::collections::HashMap;
use reqwest::{Client, Response};

pub(crate) async fn get_recommendations(token: &str) -> Result<String, reqwest::Error> {

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.gotinder.com/v2/recs/core?locale=en")
        .header("x-auth-token", format!("{}", token))
        .send()
        .await?;

    if let Err(err) = response.error_for_status_ref() {
        return Err(err);
    }

    let text = response.text().await?;
    Ok(text)
}

pub(crate) async fn like(token: &str, id: &str, photo_id: &str, s_number: u64) -> Result<String, reqwest::Error> {

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("s_number", s_number.to_string());
    map.insert("liked_content_id", photo_id.to_string());
    map.insert("liked_content_type", "photo".to_string());

    let client: Client = reqwest::Client::new();
    let response: Response= client
        .post(format!("https://api.gotinder.com/like/{}", id))
        .header("x-auth-token", format!("{}", token))
        .json(&map)
        .send()
        .await?;

    if let Err(err) = response.error_for_status_ref() {
        return Err(err);
    }

    let text: String = response.text().await?;
    Ok(text)
}