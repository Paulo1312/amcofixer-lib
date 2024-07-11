use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RootJSON{
    containers: Vec<ContainerJSON>,
    default_container: String,
    description: String,
    dns1: String,
    dns2: String,
    host_name: String,
}

#[derive(Serialize, Deserialize)]
struct ContainerJSON {
    container: String,
    cloak: CloakRootJSON,
    openvpn: OpenVpnJSON,
    shadowsocks: ShadowSocksJSON,
}

#[derive(Serialize, Deserialize)]
struct CloakRootJSON {
    last_config: String,
    port: String,
    transport_proto: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenVpnJSON {
    pub last_config: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShadowSocksJSON {
    pub last_config: String
}

fn parser_main(old_config: &str) -> Result<RootJSON> {
    let new_config: RootJSON = serde_json::from_str(old_config)?;
    Ok(new_config)
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CloakJSON {
    pub browser_sig: String,
    pub encryption_method: String,
    pub num_conn: u32,
    pub proxy_method: String,
    pub public_key: String,
    pub remote_host: String,
    pub remote_port: String,
    pub server_name: String,
    pub stream_timeout: u32,
    pub transport: String,
    pub u_i_d: String,
}

pub fn parser_cloak(old_config: &str) -> Result<CloakJSON> {
    let new_config: CloakJSON = serde_json::from_str(&old_config)?;
    Ok(new_config)
}


#[derive(Serialize, Deserialize, Clone)]
pub struct AmneziaJSON{
    pub containers: Vec<ContainerAmneziaJSON>,
    pub default_container: String,
    pub description: String,
    pub dns1: String,
    pub dns2: String,
    pub host_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContainerAmneziaJSON {
    pub container: String,
    pub cloak: CloakST,
    pub openvpn: OpenVpnJSON,
    pub shadowsocks: ShadowSocksJSON,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CloakST {
    pub last_config: CloakJSON,
    pub port: String,
    pub transport_proto: String
}
impl AmneziaJSON{
    pub fn new_from_str(old_config: &str) -> Result<AmneziaJSON>{
        let new_root_config = parser_main(&old_config)?;
        let new_cloak_config = parser_cloak(&new_root_config.containers[0].cloak.last_config)?;
        let new_cloak_st = CloakST {
            last_config: new_cloak_config,
            port: new_root_config.containers[0].cloak.port.clone(),
            transport_proto: new_root_config.containers[0].cloak.transport_proto.clone(),
        };
        let new_amnezia_config = AmneziaJSON {
            containers: vec![ContainerAmneziaJSON{
                container: new_root_config.containers[0].container.clone(),
                cloak: new_cloak_st,
                openvpn: new_root_config.containers[0].openvpn.clone(),
                shadowsocks: new_root_config.containers[0].shadowsocks.clone(),
            }],
            default_container: new_root_config.default_container,
            description: new_root_config.description,
            dns1: new_root_config.dns1,
            dns2: new_root_config.dns2,
            host_name: new_root_config.host_name
        };
        Ok(new_amnezia_config)
    }   

    pub fn to_string1(self) -> Result<String> {

        let new_cloak_st = CloakRootJSON {
            last_config: serde_json::to_string(&self.containers[0].cloak.last_config)?,
            port: self.containers[0].cloak.port.clone(),
            transport_proto: self.containers[0].cloak.transport_proto.clone(),
        };
        let export_cloak_config = RootJSON {
            containers: vec![
                ContainerJSON{
                    container: self.containers[0].container.clone(),
                    cloak: new_cloak_st,
                    openvpn: self.containers[0].openvpn.clone(),
                    shadowsocks: self.containers[0].shadowsocks.clone()
                }
            ],
            default_container: self.default_container,
            description: self.description,
            dns1: self.dns1,
            dns2: self.dns2,
            host_name: self.host_name
        };
        let return_string = serde_json::to_string(&export_cloak_config)?;
        Ok(return_string)
    }
}