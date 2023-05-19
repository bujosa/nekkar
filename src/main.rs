#![allow(unused)]
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};

type DB = (Datastore, Session);

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new database and session in memory.
    let db: &DB = &(
        Datastore::new("memory").await?,
        Session::for_db("test", "test"),
    );
    let (ds, session) = db;

    // Create a new task
    let sql = "CREATE task:1 SET title = 'test', done = false";
    let response = ds.execute(sql, &session, None, false).await?;

    let sql = "CREATE task:2 SET title = 'test2', done = false";
    let response = ds.execute(sql, &session, None, false).await?;

    // Get the task
    let sql = "SELECT * FROM task WHERE title = 'test2'";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);

    // Update the task
    let sql = "UPDATE task:2 SET done = true";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);

    // Delete the task
    let sql = "DELETE task:2";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);

    // Get all tasks
    let sql = "SELECT * FROM task";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);

    Ok(())
}
