#[derive(Serialize, Deserialize, Queryable)]
pub struct StatTypes {
    pub id: i8,
    pub name: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Stats {
    pub id: i8,
    pub node: i8,
    pub stat_type: i8,
    pub value: i8,
}
