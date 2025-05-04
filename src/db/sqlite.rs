use leptos::{server, server_fn::ServerFnError};
use serde::{Deserialize, Serialize};

#[cfg(feature="ssr")]
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[cfg(feature="ssr")]
pub async fn connetct() -> Result<Pool<Sqlite>, ServerFnError> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("db/todos.db")
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
    //Ok(SqliteConnection::connect("sqlite:Todos.db").await?)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: u16,
    pub title: String,
    completed: bool,
}

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let conn = connetct().await?;
    let mut rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos").fetch(&conn);
    let mut tods = Vec::new();

    use futures::TryStreamExt;

    while let Some(row) = rows.try_next().await? {
        tods.push(row);
    }

    Ok(tods)
}

#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let conn = connetct().await?;
    
    #[cfg(feature = "ssr")]
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    let conn = connetct().await?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&conn)
        .await
        .map(|_| ())?)
}
