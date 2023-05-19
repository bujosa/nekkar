#![allow(unused)]
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Datastore::new("memory").await?;

    let session = Session::for_db("test", "test");

    // Create a new task
    let sql = "CREATE task:1 SET title = 'test', done = false";
    let response = ds.execute(sql, &session, None, false).await?;

    let sql = "CREATE task:2 SET title = 'test2', done = false";
    let response = ds.execute(sql, &session, None, false).await?;

    // Get the task
    let sql = "SELECT * FROM task WHERE title = 'test2'";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);

    Ok(())
}
