use sqlx::{Pool, Postgres, query, postgres::PgRow, Row};

use crate::Task;

pub async fn read_tasks(pool: &Pool<Postgres>) -> Vec<Task>{
    let tasks = query(r#"
        SELECT * 
        FROM Task
    "#)
    .map(|r: PgRow| Task{
        title: r.get("title"),
        description: r.get("description"),
    })
    .fetch_all(pool)
    .await;
    return tasks.unwrap_or(vec![])
}
pub async fn insert_task(task: Task, pool: &Pool<Postgres>){
    let row = query("INSERT INTO Task(title, description) VALUES ($1, $2)").bind(task.title).bind(task.description).execute(pool).await;
    match row{
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}