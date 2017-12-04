use schema::origins;

#[derive(Debug, Serialize, Queryable)]
pub struct Origin {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "origins"]
pub struct NewOrigin {
    pub name: String,
}
