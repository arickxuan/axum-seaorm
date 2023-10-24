
use std::sync::Arc;
use once_cell::sync::Lazy;

pub static RT: Lazy<Arc<tokio::runtime::Runtime>> = Lazy::new(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    Arc::new(rt)
});

///  密码加密
pub fn encrypt_password(password: &str, salt: &str) -> String {
    use std::fmt::Write;
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();

    let mut result = String::new();
    for a in digest.iter() {
        write!(result, "{:02x}", a).unwrap();
    }
    result
}
