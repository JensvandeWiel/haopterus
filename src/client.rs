
//Structs

pub struct Client {
    pub base_url: String,
    pub api_key: String,
    pub http_client: reqwest::Client
}


impl Client {
    pub fn new(mut url: String, key: String) -> Client {
        if url.ends_with("/") {
            url.pop();
        }

        return Client {base_url: url, api_key: key, http_client: reqwest::Client::new() };
    }
}
