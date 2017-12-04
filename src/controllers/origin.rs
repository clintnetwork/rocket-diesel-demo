use diesel;
use db;
use models::origin::*;
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::Json;

#[get("/world")]
fn world(conn: db::DbConn) -> Json<Vec<Origin>> {
    use schema::origins::dsl::*;

    let results = origins.limit(100).load::<Origin>(&*conn).unwrap();
    Json(results)
}

#[post("/hello", format = "application/json", data = "<origin>")]
fn hello(conn: db::DbConn, origin: Json<NewOrigin>) -> Json<Origin> {
    use schema::origins;

    Json(
        diesel::insert_into(origins::table)
            .values(&origin.into_inner())
            .get_result(&*conn)
            .expect("Error saving new post"),
    )
}
