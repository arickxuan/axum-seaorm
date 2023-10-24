use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectOptions, ConnectionTrait, Database, DatabaseConnection,
    DbErr, EntityTrait, Statement,
};
use std::time::Duration;
// use sea_orm_migration::SchemaManager;
use sea_orm_migration::prelude::*;
use tokio::sync::OnceCell;

use migration::Migrator;

use super::entities::{prelude::*, *};

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    //let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let mut opt = ConnectOptions::new(&config::CONFIG.database.link);
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    let db = Database::connect(opt).await.expect("数据库打开失败");
    //tracing::info!("Database connected");
    db
}

async fn ping() {
    let db = DB.get_or_init(db_conn).await;
    db.ping();
}

async fn migration() -> Result<(), DbErr> {
    //    let db = Database::connect(DATABASE_URL).await?;
    //
    //    db.execute(Statement::from_string(
    //        db.get_database_backend(),
    //        format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
    //    )).await?;
    //    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    //    let  db: sea_orm::DatabaseConnection = Database::connect(&url).await?;
    //
    //    let schema_manager = SchemaManager::new(&db); // To investigate the schema
    //
    //    Migrator::refresh(&db).await?;
    //    assert!(schema_manager.has_table("bakery").await?);
    //    assert!(schema_manager.has_table("chef").await?);

    //db.execute(stmt)

    // let db = &match db.get_database_backend() {
    //     DbBackend::MySql => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
    //         )).await?;
    //
    //         let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    //         Database::connect(&url).await?
    //     },
    //     _ => println!("something else"),
    // };

    Ok(())
}

async fn save() -> Result<(), DbErr> {
    let db = DB.get_or_init(db_conn).await;
    let happy_bakery = user::ActiveModel {
        username: ActiveValue::Set("Bakery".to_owned()),
        telephone: ActiveValue::Set(139),
        gender: ActiveValue::Set(1.to_string()),
        nickname: ActiveValue::Set("bBakery".to_owned()),
        ..Default::default()
    };
    let res = User::insert(happy_bakery).exec(db).await?;

    Ok(())
}

pub async fn find() -> Result<(), DbErr> {
    let db = DB.get_or_init(db_conn).await;
    let sad_bakery = User::find()
        //.filter(User::Column::Username.eq("arick"))
        .one(db)
        .await?;
    println!("{sad_bakery:#?}");
    Ok(())
}

async fn update() -> Result<(), DbErr> {
    let db = DB.get_or_init(db_conn).await;
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(2),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(db).await?;
    Ok(())
}

async fn del() -> Result<(), DbErr> {
    let db = DB.get_or_init(db_conn).await;
    let john = user::ActiveModel {
        id: ActiveValue::Set(2), // The primary key must be set
        ..Default::default()
    };
    john.delete(db).await?;
    Ok(())
}
