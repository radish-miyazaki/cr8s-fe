use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;

use super::APP_HOST;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn api_rustaceans(token: &String) -> Result<Vec<Rustacean>, gloo_net::Error> {
    let resposne = Request::get(&format!("{}/rustaceans", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    resposne.json::<Vec<Rustacean>>().await
}

pub async fn api_rustacean_create(
    token: &String,
    name: String,
    email: String,
) -> Result<Rustacean, gloo_net::Error> {
    let resposne = Request::post(&format!("{}/rustaceans", APP_HOST))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&json!({ "name": name, "email": email }))?
        .send()
        .await?;

    resposne.json::<Rustacean>().await
}
