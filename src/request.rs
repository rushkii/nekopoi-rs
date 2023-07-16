use reqwest::{header::{HeaderMap, HeaderValue, USER_AGENT}, Response};


fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("appbuildcode", HeaderValue::from_static("25030"));
    headers.insert("appsignature", HeaderValue::from_static("pOplm8IDEDGXN55IaYohQ8CzJFvWsfXyhGvwPRD9kWgzYSRuuvAOPfsE0AJbHVbAJyWGsGCNUIuQLJ7HbMbuFLMWwDgHNwxOrYMH"));
    headers.insert("token", HeaderValue::from_static("XbGSFkQsJYbFC6pcUMCFL4oNHULvHU7WdDAXYgpmqYlh7p5ZCQ4QZ13GDgowiOGvAejz9X5H6DYvEQBMrc3A17SO3qwLwVkbn6YY"));
    headers.insert(USER_AGENT, HeaderValue::from_static("okhttp/4.9.0"));
    headers
}

pub async fn request_api(
    endpoint: &str,
    params: Option<&[(&str, &str)]>,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", "https://cu8auck2lc.3z094n2681i06q8k14w31cu4q80d5p.com/330cceade91a6a9cd30fb8042222ed56/71b8acf33b508c7543592acd9d9eb70d", endpoint);
    let req = client.get(&url).headers(get_headers());

    let req = match params {
        Some(params) => req.query(params),
        None => req,
    };

    let res = req.send().await?;
    Ok(res)
}
