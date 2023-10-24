use sqlx::{FromRow, MySql, MySqlPool, Pool};

use sqlx::mysql::MySqlPoolOptions;

pub async fn mysql() -> Result<Vec<User>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:3308/habit")
        .await?;

    let rows = sqlx::query_as::<_, User>("SELECT id, username,telephone FROM user")
        .fetch_all(&pool)
        .await
        .unwrap();

    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!("{rows:?}");
    Ok(rows)

    // match pool {
    //     Ok(pools) => {
    //         let row = sqlx::query_as("SELECT ?")
    //             .bind(150_i64)
    //             .fetch_one(&pools).await;

    //         match row {
    //             Ok(r) => {
    //                 println!("ok {}",r);
    //             }
    //             Err(E) => println!("err row"),
    //         }

    //         //assert_eq!(row.0, 150);
    //     }
    //     Err(E) => println!("err"),
    // }
}

#[derive(Debug, FromRow)]
pub struct User {
    id: i32,
    username: String,
    telephone: i32,
}
