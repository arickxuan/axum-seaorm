use serde::Deserialize;
use serde::Serialize;

///  用户登录
#[derive(Deserialize, Debug)]
pub struct UserLoginReq {
    ///  用户名
    pub username: String,
    ///  用户密码
    pub password: String,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct UserSearchReq{
    //  用户名
    pub username: Option<String>,
    pub status: Option<String>,
    pub telephone: Option<String>,
}