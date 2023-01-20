use reqwest;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use crate::client::Client;


//Structs

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
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







//Function

// tokio let's us use "async" on our main function
pub async fn list(cli: Client) -> Result<Vec<Server>, reqwest::Error> {

    let response: Root = cli.http_client
        .get("https://stoplight.io/mocks/pterodactyl-api/api/19936968/")
        .header("Authorization", "Bearer 123".to_owned())
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