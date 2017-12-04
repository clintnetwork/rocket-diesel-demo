use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
use dotenv::dotenv;
use std;

// An alias to the type for a pool of Diesel PgConnection connections.
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").expect(
        "DATABASE_URL must be set",
    ));
    r2d2::Pool::new(manager).expect("db pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
