use serde::de::DeserializeOwned;

pub async fn get<'a, T>(url: String) -> Result<T, reqwest::Error> where T: DeserializeOwned + Clone {
    let response = reqwest::get(
        &url
    )
        .await?
        .text()
        .await?;

    let b = serde_json::from_str(&response).unwrap();

    Ok(b)
}
