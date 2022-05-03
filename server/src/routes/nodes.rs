use diesel::types::IsNull::No;
use diesel::{self, prelude::*};
use rocket_contrib::json::Json;

use crate::models::nodes::{InsertableNodes, Nodes};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[post("/nodes", data = "<nodes>")]
pub fn create_node_view(
    conn: DbConn,
    nodes: Json<InsertableNodes>,
) -> Result<Json<Vec<Nodes>>, String> {
    let inserted_rows = diesel::insert_into(schema::nodes::table)
        .values(&nodes.0)
        .get_results(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })
        .map(Json::<Vec<Nodes>>)?;

    Ok(inserted_rows)
}

#[get("/nodes")]
pub fn list_node_view(conn: DbConn) -> Result<Json<Vec<Nodes>>, String> {
    use crate::schema::nodes::dsl::*;
    nodes
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}
