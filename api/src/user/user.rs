use std::io::Bytes;
use axum::http::HeaderMap;
use axum::{Form, Json};
use axum::extract::{DefaultBodyLimit, Query, Multipart, State, Path};
use axum::extract::multipart::Field;
use axum::response::Html;
use chrono::Utc;
use futures::TryStreamExt;
use tokio::io::AsyncWriteExt;
use common::ctx::jwt::AuthBody;
use common::ctx::res::Res;
use common::ctx::{res::{ListData,PageParams}};
use common::db::{DB, db_conn};
use common::models::user::UserLoginReq;
use common::models::user::UserSearchReq;
use common::entities::{user::Entity as UserEntity,user::Model as UserModel, user};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_util::bytes::Buf;
use service;
use crate::AppState;
use config::CONFIG;


/// 用户登录

pub async fn login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let db = DB.get_or_init(db_conn).await;
    match service::login(db, login_req, header).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn search_user(Query(page_params): Query<PageParams>,Query(req): Query<UserSearchReq> ) -> Res<ListData<UserModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::getUsers(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn upload(Path(id): Path<i32>) -> Html<&'static str> {
    println!("{id}");
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/comm/do/upload" method="post" enctype="multipart/form-data">

                    <label>
                        Upload file:{id}
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

pub async fn accept_upload(mut multipart: Multipart,)-> Json<res<Vec<String>>> {
    let mut files: Vec<(String )> = Vec::new();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = &field.bytes().await.unwrap();
        let extension = match filename.rsplit('.').next() {
            Some(ext) => ext,
            _ => "unknown",
        };

        println!("Length of `` (`{filename}`: `{content_type}`) is {}  bytes", data.len());


        /// 写入文件
        let path = &CONFIG.server.upload_path;
        // 创建目标文件的路径
        let id = md5::compute(data);
        let unique_filename = format!("{:x}.{}",  id, extension);
        let date_path = Utc::now().format("%Y/%m/%d").to_string();
        let upload_dir = format!("{}/{}", path,date_path);

        fs::create_dir_all(&upload_dir).await.unwrap();

        let upload_path = format!("{}/{}", upload_dir, unique_filename);

        // 创建目标文件并写入数据
        let mut file = match fs::File::create(&upload_path).await {
            Ok(file) => file,
            _ => panic!("ooo"),
        };

        if let Err(_) = file.write_all(data.as_ref()).await {
            println!("err data");
        }
        let uri = format!("{}/{}", &CONFIG.server.static_prefix, upload_path);
        files.push(uri);


    }

    let r = res{
        code: 0,
        msg: "ok".to_string(),
        data: files,
    };

    Json(r)


}

/// 中文响应
async fn cn(msg: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().unwrap(),
    );
    Ok((headers, msg))
}

pub async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/comm/do/from" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Serialize,Debug)]
#[allow(dead_code)]
pub struct Input {
    name: String,
    email: String,
}

#[derive(Deserialize, Serialize,Debug)]
#[allow(dead_code)]
pub struct res<T> {
    code: u64,
    msg: String,
    data:T,
}

pub async fn accept_form(Form(input): Form<Input>) -> Json<Input> {
    dbg!(&input);
    Json(input)
}