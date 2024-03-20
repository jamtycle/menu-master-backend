#[derive(Debug, Serialize, Deserialize)]
pub struct Restaurant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    pub cuisine: String,
    pub profit_rate: f64,
    pub active: bool,
    pub deleted: bool,
}