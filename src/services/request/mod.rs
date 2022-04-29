use serde::de::DeserializeOwned;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn just_get(url: String) -> Result<String, reqwest::Error> {
    let res = CLIENT.get(&url)
        .send()
        .await;

    match res {
        Ok(res) => res
            .text()
            .await,
        Err(err) => Err(err)
    }
}

pub async fn deserialize<'a, T>(response: String) -> anyhow::Result<T> where T: DeserializeOwned + Clone {
    Ok(serde_json::from_str(&response)?)
}

pub async fn get<'a, T>(url: String) -> anyhow::Result<T> where T: DeserializeOwned + Clone {
    let response = just_get(url.to_owned()).await?;

    deserialize::<T>(response).await
}

// pub async fn parse_get<'a, T>(url: String, middleware: &dyn Fn(String) -> String) -> Result<T, reqwest::Error> where T: DeserializeOwned + Clone {
//     let parse = middleware.to_owned();
//     let response = just_get(url).await?;

//     Ok(serde_json::from_str(&parse(response)).unwrap())
// }
