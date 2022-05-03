use crate::schema::nodes;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Nodes {
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "nodes"]
pub struct InsertableNodes {
    pub name: String,
    pub address: String,
}
