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
    let task1 = create_task(db, "Refactor the code").await?;
    create_task(db, "Test the code").await?;
    create_task(db, "Document the code").await?;

    // Get all tasks
    get_all_task(db).await?;

    // Get a task

    Ok(())
}

async fn create_task(db: &DB, title: &str) -> Result<Task> {
    let (ds, session) = db;
    let sql = format!("CREATE task SET title = '{}', done = false", title);
    let res = ds.execute(&sql, &session, None, false).await?;

    let id = into_iter_objects(res)?
        .next()
        .transpose()?
        .and_then(|obj| obj.get("id").map(|id| id.to_string()))
        .ok_or_else(|| anyhow!("No id returned."));

    // Return the Task
    let task = Task {
        id: id?,
        title: title.to_string(),
        done: false,
    };

    Ok(task)
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

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("A record was not an Object")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No records found.")),
    }
}
