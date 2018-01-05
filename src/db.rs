use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::fmt;
use std::net::{Ipv4Addr, IpAddr};
use std::ops::Deref;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};

#[derive(Debug, Deserialize)]
pub struct DbConfig {
  pub host: IpAddr,
  pub port: u16,
  pub user: String,
  pub password: Option<String>,
  pub database: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5432,
            user: String::from("postgres"),
            password: None,
            database: String::from(""),
        }
    }
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut connect = format!("postgres://{}", self.user);
        connect = match self.password {
            Some(ref p) => {
                // We can potentially get non-url friendly chars here so we need to encode them
                let encoded_password = utf8_percent_encode(p, PATH_SEGMENT_ENCODE_SET).to_string();
                format!("{}:{}", connect, encoded_password)
            }
            None => connect,
        };
        connect = format!("{}@{}:{}/{}", connect, self.host, self.port, self.database);
        write!(f, "{}", connect)
    }
}

// An alias to the type for a pool of Diesel PgConnection connections.
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool(config: DbConfig) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(config.to_string());
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

