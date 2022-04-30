use serde::de::DeserializeOwned;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn just_get(url: String) -> Result<String, reqwest::Error> {
    let mut bypasser = cloudflare_bypasser::Bypasser::default()
        .retry(5)
        .wait(5)
        .random_user_agent(true);

    // to pass the verify both of the cookie and user agent are needed
    let (cookie, user_agent);
    loop {
        if let Ok((c, ua)) =  bypasser.bypass(&url) {
            cookie = c;
            user_agent = ua;
            break;
        }
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::COOKIE, cookie);
    headers.insert(reqwest::header::USER_AGENT, user_agent);

    let res = CLIENT.get(&url)
        .headers(headers)
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
