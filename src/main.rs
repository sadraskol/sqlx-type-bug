use serde::Deserialize;

mod types;

#[derive(Clone, Debug, sqlx::FromRow, Deserialize)]
struct Speaker {
    id: i32,
    name: String,
    occupation: Vec<types::Occupation>,
}

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://postgres:mysecretpassword@localhost:5432/public")
        .await.unwrap();
    let res = sqlx::query_as::<_, Speaker>("select id, name, occupation from speaker")
        .fetch_all(&pool)
        .await.unwrap();
    println!("{:?}", res);
}
