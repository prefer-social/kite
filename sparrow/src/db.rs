// Database(sqlite) abstraction
// TODO: Optional choice either using local sqliet or turso libsql

// use libsql_client::ResultSet;
use spin_sdk::sqlite::QueryResult;
// use spin_sdk::sqlite::RowResult;
use spin_sdk::sqlite::Value;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

#[derive(Debug)]
pub enum DbConf {
    Local,
    Turso(String, String),
}

impl DbConf {
    pub async fn new() -> DbConf {
        // if std::env::var("TURSO_URL").is_ok() && std::env::var("TURSO_TOKEN").is_ok() {
        //     // This is Turso
        //     return DbConf::Turso(
        //         std::env::var("TURSO_URL").unwrap(),
        //         std::env::var("TURSO_TOKEN").unwrap(),
        //     );
        // } else {
        //     return DbConf::Local;
        // }

        return DbConf::Local;
    }
}

#[derive(Debug)]
pub enum Db {
    Local(spin_sdk::sqlite::Connection),
    //Turso(libsql_client::Client),
}

impl Db {
    pub async fn new(dbconf: &DbConf) -> Db {
        match dbconf {
            DbConf::Local => {
                let connection = spin_sdk::sqlite::Connection::open_default().unwrap();
                return Db::Local(connection);
            }
            DbConf::Turso(turso_url, turso_auth_token) => {
                // let client = libsql_client::Client::from_config(libsql_client::Config {
                //     url: url::Url::parse(turso_url.to_owned().as_str()).unwrap(),
                //     auth_token: Some(turso_auth_token.to_owned().to_string()),
                // })
                // .await
                // .unwrap();

                // return Db::Turso(client);
                let connection = spin_sdk::sqlite::Connection::open_default().unwrap();
                return Db::Local(connection);
            }
        }
    }
}

// TODO: Rewrite DB execute with Connection with Builder pattern. So make it one liner.
#[derive(Debug)]
pub enum Connection {
    Local(spin_sdk::sqlite::Connection),
    //Turso(libsql_client::Client),
}

impl Connection {
    // This method will help users to discover the builder
    pub async fn builder() -> ConnectionBuilder {
        ConnectionBuilder::build().await
    }
}

pub enum ConnectionBuilder {
    Local(spin_sdk::sqlite::Connection),
    //Turso(libsql_client::Client),
}

impl ConnectionBuilder {
    pub async fn build() -> ConnectionBuilder {
        // if std::env::var("TURSO_URL").is_ok() && std::env::var("TURSO_TOKEN").is_ok() {
        //     let client = libsql_client::Client::from_config(libsql_client::Config {
        //         url: url::Url::parse(std::env::var("TURSO_URL").unwrap().to_owned().as_str())
        //             .unwrap(),
        //         auth_token: Some(std::env::var("TURSO_TOKEN").unwrap().to_owned().to_string()),
        //     })
        //     .await
        //     .unwrap();
        //     return ConnectionBuilder::Turso(client);
        // } else {
        let connection = spin_sdk::sqlite::Connection::open_default().unwrap();
        return ConnectionBuilder::Local(connection);
        //}
    }

    pub async fn execute(self, q: &str, query_params: &[Value]) -> QueryResult {
        match self {
            ConnectionBuilder::Local(c) => {
                tracing::debug!(q);
                tracing::debug!("{query_params:?}");

                // https://discord.com/channels/926888690310053918/950022897160839248/1211210047489843241
                return c
                    .execute(q, query_params)
                    .expect("db execution error - annoying!!!");
            } // ConnectionBuilder::Turso(c) => {
              //     // TODO: Convert spin_sdk::sqlite::Value to use sqlite::Value;

              //     let mut legacy_val: Vec<libsql_client::Value> = Vec::new();
              //     for v in query_params {
              //         let c: libsql_client::Value = from_legacy_value(v).await;
              //         legacy_val.push(c);
              //     }
              //     let a = legacy_val.as_slice();

              //     let stmt = match a.len() {
              //         0 => libsql_client::Statement::new(q),
              //         _ => libsql_client::Statement::with_args(q, a),
              //     };

              //     let rs = c
              //         .execute(stmt)
              //         .await
              //         .expect("Getting error from Turso ResultSet");
              //     return resultset_to_queryresult(rs).await;
              // }
        }
    }
}

// pub async fn resultset_to_queryresult(rs: ResultSet) -> QueryResult {
//     // From https://fermyon.github.io/rust-docs/spin/main/spin_sdk/sqlite/struct.QueryResult.html
//     // To https://docs.rs/libsql-client/latest/libsql_client/struct.ResultSet.html

//     // pub struct QueryResult {
//     //    pub columns: Vec<String>,
//     //    pub rows: Vec<RowResult>,
//     // }
//     // pub struct RowResult {
//     //    pub values: Vec<Value>,
//     // }

//     // pub struct ResultSet {
//     //     pub columns: Vec<String>,
//     //     pub rows: Vec<Row>,
//     //     pub rows_affected: u64,
//     //     pub last_insert_rowid: Option<i64>,
//     // }=
//     // pub struct Row {
//     //     pub values: Vec<Value>,
//     //     pub value_map: HashMap<String, Value>,
//     // }

//     let columns = rs.columns;
//     let mut my_rows: Vec<RowResult> = Vec::new();

//     // let rows: Vec<Value> = rs.rows.into_iter().map(|x| async {
//     //     x.values
//     //         .into_iter()
//     //         .map(|y| async { convert_values(y).await })
//     // });

//     for row in rs.rows.into_iter() {
//         let mut row_result_values = Vec::new();
//         for v in row.values.into_iter() {
//             let spin_sqlite_value = convert_values(v).await;
//             row_result_values.push(spin_sqlite_value);
//         }
//         my_rows.push(RowResult {
//             values: row_result_values,
//         });
//     }

//     QueryResult {
//         columns,
//         rows: my_rows,
//     }
// }

// pub async fn convert_values(libsql_value: libsql_client::Value) -> spin_sdk::sqlite::Value {
//     // Enum libsql_client::Value
//     // pub enum Value {
//     //     Null,
//     //     Integer {
//     //         value: i64,
//     //     },
//     //     Float {
//     //         value: f64,
//     //     },
//     //     Text {
//     //         value: String,
//     //     },
//     //     Blob {
//     //         value: Vec<u8>,
//     //     },
//     // }
//     //
//     // Enum spin_sdk::sqlite::Value
//     // pub enum Value {
//     //     Integer(i64),
//     //     Real(f64),
//     //     Text(String),
//     //     Blob(Vec<u8>),
//     //     Null,
//     // }

//     match libsql_value {
//         libsql_client::Value::Integer { value: x } => return spin_sdk::sqlite::Value::Integer(x),
//         libsql_client::Value::Float { value: x } => return spin_sdk::sqlite::Value::Real(x),
//         libsql_client::Value::Text { value: x } => return spin_sdk::sqlite::Value::Text(x),
//         libsql_client::Value::Blob { value: x } => return spin_sdk::sqlite::Value::Blob(x),
//         libsql_client::Value::Null => return spin_sdk::sqlite::Value::Null,
//     }
// }

// async fn from_legacy_value(value: &Value) -> libsql_client::Value {
//     match value {
//         spin_sdk::sqlite::Value::Integer(i) => libsql_client::Value::Integer { value: *i },
//         spin_sdk::sqlite::Value::Real(r) => libsql_client::Value::Float { value: *r },
//         spin_sdk::sqlite::Value::Text(t) => libsql_client::Value::Text { value: t.clone() },
//         spin_sdk::sqlite::Value::Blob(b) => libsql_client::Value::Blob { value: b.clone() },
//         spin_sdk::sqlite::Value::Null => libsql_client::Value::Null,
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[tokio::test]
//     async fn it_works() {
//         let dbconf = DbConf::Local;
//         let db = Db::new(dbconf).await;

//         //dbcon.execute("select 1");
//     }
// }
