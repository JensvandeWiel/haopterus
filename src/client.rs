

//Structs

use std::borrow::Borrow;
use url::Url;
use reqwest;
use reqwest::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub struct Client {
    pub url: Url,
    pub api_key: String,
    pub http_client: reqwest::Client
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerRoot {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub object: String,
    pub attributes: Server,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "server_owner")]
    pub server_owner: bool,
    pub identifier: String,
    #[serde(rename = "internal_id")]
    pub internal_id: i64,
    pub uuid: String,
    pub name: String,
    pub node: String,
    #[serde(rename = "sftp_details")]
    pub sftp_details: SftpDetails,
    pub description: String,
    pub limits: Limits,
    pub invocation: String,
    #[serde(rename = "docker_image")]
    pub docker_image: String,
    #[serde(rename = "egg_features")]
    pub egg_features: Vec<String>,
    #[serde(rename = "feature_limits")]
    pub feature_limits: FeatureLimits,
    pub status: Value,
    #[serde(rename = "is_suspended")]
    pub is_suspended: bool,
    #[serde(rename = "is_installing")]
    pub is_installing: bool,
    #[serde(rename = "is_transferring")]
    pub is_transferring: bool,
    pub relationships: Relationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SftpDetails {
    pub ip: String,
    pub port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Limits {
    pub memory: i64,
    pub swap: i64,
    pub disk: i64,
    pub io: i64,
    pub cpu: i64,
    pub threads: Value,
    #[serde(rename = "oom_disabled")]
    pub oom_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureLimits {
    pub databases: i64,
    pub allocations: i64,
    pub backups: i64,
}



//Relationships
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    pub allocations: Allocations,
    pub variables: Variables,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocations {
    pub data: Vec<Allocation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub object: String,
    pub attributes: AllocationAttributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationAttributes {
    pub id: i64,
    pub ip: String,
    #[serde(rename = "ip_alias")]
    pub ip_alias: Value,
    pub port: i64,
    pub notes: Value,
    #[serde(rename = "is_default")]
    pub is_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub object: String,
    pub data: Vec<Variable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub object: String,
    pub attributes: VariableAttributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributes {
    pub name: String,
    pub description: String,
    #[serde(rename = "env_variable")]
    pub env_variable: String,
    #[serde(rename = "default_value")]
    pub default_value: String,
    #[serde(rename = "server_value")]
    pub server_value: String,
    #[serde(rename = "is_editable")]
    pub is_editable: bool,
    pub rules: String,
}
//

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRoot {
    pub object: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub admin: bool,
    pub username: String,
    pub email: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    pub language: String,
}

impl Client {
    pub fn new(url: String, key: String) -> Client {
        let parsed_url = Url::parse(&*url).expect("Failed parsing url");
        return Client {url: parsed_url, api_key: key, http_client: reqwest::Client::new() };
    }

    //lists all servers
    pub async fn list(&self) -> Result<Vec<Server>, Error> {

        let token: String = "Bearer: ".to_string() + self.api_key.borrow();

        let response: ServerRoot = self.http_client
            .get(self.url.join("/api/client/").expect("Couldn't parse url"))
            .header("Authorization", token)
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        let mut servers: Vec<Server> = vec![];
        for server in response.data {
            servers.push(server.attributes);
        }
        Ok(servers)
    }

    // returns details of user
    pub async fn user_details(&self) -> Result<User, Error> {
        let token: String = "Bearer: ".to_string() + self.api_key.borrow();

        let response: UserRoot = self.http_client
            .get(self.url.join("/api/client/account").expect("Couldn't parse url"))
            .header("Authorization", token)
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(response.user)
    }
}
