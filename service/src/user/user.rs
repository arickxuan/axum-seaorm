use anyhow::anyhow;
use anyhow::Result;
use axum::http::HeaderMap;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*,};
use common::ctx::jwt::{AuthBody, AuthPayload};
use common::ctx::{res::{ListData,PageParams}};
use common::utils;
use common::entities::{user::Entity as UserEntity,user::Model as UserModel, user};
use common::models::user::{UserLoginReq,UserSearchReq};


// 用户登录
pub async fn login(db: &DatabaseConnection, login_req: UserLoginReq, header: HeaderMap) -> Result<AuthBody> {
    let mut msg = "登录成功".to_string();
    let mut status = "1".to_string();

    //UserModel::find();

    // 根据用户名获取用户信息
    let user_info = match UserEntity::find().filter(user::Column::Username.eq(login_req.username.clone())).one(db).await? {
        Some(user_info) => {
            if user_info.status == 0 {
                msg = "用户已被禁用".to_string();
                status = "0".to_string();
                //set_login_info(header, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
                return Err(anyhow!("用户已被禁用"));
            } else {
                user_info
            }
        }
        None => {
            msg = "用户不存在".to_string();
            status = "0".to_string();
            //set_login_info(header, "".to_string(), login_req.user_name.clone(), msg.clone(), status.clone(), None, None).await;
            return Err(anyhow!("用户不存在"));
        }
    };
    //  验证密码是否正确
    if utils::encrypt_password(&login_req.password, &user_info.salt) != user_info.password {
        println!("加密后:{}",utils::encrypt_password(&login_req.password, &user_info.salt));
        println!("明文{},秘闻{},盐{}",login_req.password,user_info.password,user_info.salt);
        msg = "密码错误".to_string();
        status = "0".to_string();
        //set_login_info(header, "".to_string(), login_req.username.clone(), msg.clone(), status.clone(), None, None).await;
        return Err(anyhow!("密码不正确"));
    };
    // 注册JWT
    let claims = AuthPayload {
        id: user_info.id.clone().to_string(),               // 用户id
        name: login_req.username.clone(), // 用户名
    };
    let token_id = scru128::new_string();
    let token = common::ctx::jwt::authorize(claims.clone(), token_id.clone()).await.unwrap();
    // 成功登录后
    //  写入登录日志

    Ok(token)
}

// 用户登录

pub async fn getUsers(db: &DatabaseConnection,page_params: PageParams, req: UserSearchReq) -> Result<ListData<UserModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(u32::MAX as u64);
    //  生成查询条件
    let mut s = UserEntity::find();

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(user::Column::Status.eq(x));
        }
    }

    if let Some(x) = req.telephone {
        if !x.is_empty() {
            s = s.filter(user::Column::Telephone.eq(x));
        }
    }


    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.paginate(db, page_per_size); //order_by_asc(sys_menu::Column::OrderSort)
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        list,
        total,
        total_pages,
        page_num,
    };
    Ok(res)
}