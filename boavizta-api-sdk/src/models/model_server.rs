/*
 * BOAVIZTAPI - DEMO
 *
 * # 🎯 Retrieving the impacts of digital elements This is a quick demo, to see full documentation [click here](http://api.boavizta.org)  ## ➡️Server router  ### Server routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Cloud router  ### Cloud routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |     X    | |   ADP  |        X       |     X    | |   PE   |        X       |     X    | ## ➡️Component router  ### Component routers support the following impacts:  | Impact | 🔨 Manufacture | 🔌 Usage | |--------|----------------|----------| |   GWP  |        X       |          | |   ADP  |        X       |          | |   PE   |        X       |          |
 *
 * The version of the OpenAPI document: 0.1.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ModelServer {
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(rename = "archetype", skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
}

impl ModelServer {
    pub fn new() -> ModelServer {
        ModelServer {
            manufacturer: None,
            name: None,
            _type: None,
            year: None,
            archetype: None,
        }
    }
}