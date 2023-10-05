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
        id: r.get("taskid"),
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
pub async fn delete_task(id: u32, pool: &Pool<Postgres>){
    let status = query("DELETE FROM Task WHERE taskid=$1").bind(id as i32).execute(pool).await;
    match status{
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}