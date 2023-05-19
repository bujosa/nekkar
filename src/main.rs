#![allow(unused)]
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Datastore::new("memory").await?;

    let session = Session::for_db("test", "test");

    let sql = "CREATE task:1 SET title = 'test', done = false";

    let response = ds.execute(sql, &session, None, false).await?;

    println!("{:?}", response);

    Ok(())
}
