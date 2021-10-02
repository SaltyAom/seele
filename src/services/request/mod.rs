use serde::de::DeserializeOwned;

pub async fn just_get(url: String) -> Result<String, reqwest::Error> {
    Ok(
        reqwest::get(
            &url
        )
            .await?
            .text()
            .await?
    )
}

pub async fn get<'a, T>(url: String) -> Result<T, reqwest::Error> where T: DeserializeOwned + Clone {
    let response = just_get(url).await?;

    Ok(serde_json::from_str(&response).unwrap())
}

// pub async fn parse_get<'a, T>(url: String, middleware: &dyn Fn(String) -> String) -> Result<T, reqwest::Error> where T: DeserializeOwned + Clone {
//     let parse = middleware.to_owned();
//     let response = just_get(url).await?;

//     Ok(serde_json::from_str(&parse(response)).unwrap())
// }
