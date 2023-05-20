#![allow(unused)]
use anyhow::{anyhow, Ok, Result};
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};

type DB = (Datastore, Session);

#[derive(Debug)]
struct Task {
    id: String,
    title: String,
    done: bool,
}

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
    // create_task(db, "Test the code").await?;
    // create_task(db, "Document the code").await?;

    // Get all tasks
    get_all_task(db).await?;

    // Get a task

    Ok(())
}

async fn create_task(db: &DB, title: &str) -> Result<()> {
    let (ds, session) = db;
    let sql = format!("CREATE task SET title = '{}', done = false", title);
    let res = ds.execute(&sql, &session, None, false).await?;
    // Extract first result
    let first_result = res.into_iter().next().unwrap();

    // Extract id from first result
    let task = first_result.result?.first();

    // This is the task result task: Object(Object({"done": False, "id": Thing(Thing { tb: "task", id: String("ip943vi2jqvu0kmgczx0") }), "title": Strand(Strand("Refactor the code"))}))
    // transform task into a Task struct

    println!("task: {:?}", task);

    Ok(())
}

async fn get_all_task(db: &DB) -> Result<()> {
    let (ds, session) = db;
    let sql = "SELECT * FROM task";
    let response = ds.execute(sql, &session, None, false).await?;
    println!("response: {:?}", response);
    Ok(())
}

async fn get_task(db: &DB, id: String) -> Result<()> {
    let (ds, session) = db;
    let sql = format!("SELECT * FROM task WHERE id = {}", id);
    let response = ds.execute(&sql, &session, None, false).await?;
    println!("response: {:?}", response);
    Ok(())
}

async fn update_task(db: &DB, id: String, done: bool) -> Result<()> {
    let (ds, session) = db;
    let sql = format!("UPDATE task:{} SET done = {}", id, done);
    ds.execute(&sql, &session, None, false).await?;
    Ok(())
}

async fn delete_task(db: &DB, id: String) -> Result<()> {
    let (ds, session) = db;
    let sql = format!("DELETE task:{}", id);
    ds.execute(&sql, &session, None, false).await?;
    Ok(())
}
