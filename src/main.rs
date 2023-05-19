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
    create_task(db, "Refactor the code").await?;
    create_task(db, "Test the code").await?;
    create_task(db, "Document the code").await?;

    // // Get the task
    // let sql = "SELECT * FROM task WHERE title = 'test2'";
    // let response = ds.execute(sql, &session, None, false).await?;
    // println!("response: {:?}", response);

    // // Update the task
    // let sql = "UPDATE task:2 SET done = true";
    // ds.execute(sql, &session, None, false).await?;

    // // Delete the task
    // let sql = "DELETE task:2";
    // ds.execute(sql, &session, None, false).await?;

    // // Get all tasks
    // let sql = "SELECT * FROM task";
    // let response = ds.execute(sql, &session, None, false).await?;
    // println!("response: {:?}", response);

    Ok(())
}

async fn create_task(db: &DB, title: &str) -> Result<()> {
    let (ds, session) = db;
    let sql = format!("CREATE task SET title = '{}', done = false", title);
    ds.execute(&sql, &session, None, false).await?;
    Ok(())
}

async fn get_all_task(db: &DB) -> Result<()> {
    let (ds, session) = db;
    let sql = "SELECT * FROM task";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);
    Ok(())
}
