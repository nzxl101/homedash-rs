use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

use crate::config::APICredsConfig;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodesData {
    data: Vec<ProxmoxV2Nodes>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2Nodes {
    node: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2Data {
    node: String,
    status: ProxmoxV2NodeStatus,
    lxc: ProxmoxV2NodeLXC,
    qemu: ProxmoxV2NodeQEMU,
    storage: ProxmoxV2NodeStorage,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeStatusMemory {
    used: f32,
    total: f32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeStatusData {
    cpu: f32,
    memory: ProxmoxV2NodeStatusMemory,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeStatus {
    data: ProxmoxV2NodeStatusData,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeLXCData {
    status: String,
    name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeLXC {
    data: Vec<ProxmoxV2NodeLXCData>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeQEMUData {
    status: String,
    name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeQEMU {
    data: Vec<ProxmoxV2NodeQEMUData>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeStorageData {
    total: u64,
    used: u64,
    storage: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProxmoxV2NodeStorage {
    data: Vec<ProxmoxV2NodeStorageData>,
}

async fn req(endpoint: String, credentials: APICredsConfig) -> Result<Response, Box<dyn Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = format!("{}/api2/{}", &credentials.url, endpoint);

    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Bearer PVEAPIToken={}",
                format!("{}={}", &credentials.username, &credentials.password)
            ),
        )
        .send()
        .await?;

    Ok(response)
}

async fn get_proxmox_nodes(
    credentials: APICredsConfig,
) -> Result<ProxmoxV2NodesData, Box<dyn Error>> {
    let response = req(String::from("json/nodes"), credentials).await?;
    let data = response.json::<ProxmoxV2NodesData>().await?;

    Ok(data)
}

async fn get_proxmox_node_status(
    credentials: APICredsConfig,
    node: String,
) -> Result<ProxmoxV2NodeStatus, Box<dyn Error>> {
    let response = req(format!("json/nodes/{}/status", &node), credentials).await?;
    let data = response.json::<ProxmoxV2NodeStatus>().await?;

    Ok(data)
}

async fn get_proxmox_node_lxc(
    credentials: APICredsConfig,
    node: String,
) -> Result<ProxmoxV2NodeLXC, Box<dyn Error>> {
    let response = req(format!("json/nodes/{}/lxc", &node), credentials).await?;
    let data = response.json::<ProxmoxV2NodeLXC>().await?;

    Ok(data)
}

async fn get_proxmox_node_qemu(
    credentials: APICredsConfig,
    node: String,
) -> Result<ProxmoxV2NodeQEMU, Box<dyn Error>> {
    let response = req(format!("json/nodes/{}/qemu", &node), credentials).await?;
    let data = response.json::<ProxmoxV2NodeQEMU>().await?;

    Ok(data)
}

async fn get_proxmox_node_storage(
    credentials: APICredsConfig,
    node: String,
) -> Result<ProxmoxV2NodeStorage, Box<dyn Error>> {
    let response = req(format!("json/nodes/{}/storage", &node), credentials).await?;
    let data = response.json::<ProxmoxV2NodeStorage>().await?;

    Ok(data)
}

pub async fn get_proxmox_data(
    credentials: APICredsConfig,
) -> Result<Vec<ProxmoxV2Data>, Box<dyn Error>> {
    let mut data: Vec<ProxmoxV2Data> = Vec::new();

    let nodes = get_proxmox_nodes(credentials.clone()).await?;
    for server in nodes.data.into_iter() {
        let status = get_proxmox_node_status(credentials.clone(), server.node.clone()).await?;
        let lxc = get_proxmox_node_lxc(credentials.clone(), server.node.clone()).await?;
        let qemu = get_proxmox_node_qemu(credentials.clone(), server.node.clone()).await?;
        let storage = get_proxmox_node_storage(credentials.clone(), server.node.clone()).await?;

        data.push(ProxmoxV2Data {
            node: String::from(&server.node),
            status,
            lxc,
            qemu,
            storage,
        });
    }

    Ok(data)
}
