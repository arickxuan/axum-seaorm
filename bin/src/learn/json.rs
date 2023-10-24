
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize, Debug)]
struct User{
    username:String,
    password:String,
}

pub fn toString() -> Result<()>{
    let john = User {
        username: "John".to_string(),
        password: "30".to_string(),
    };

    // 将结构体序列化为 JSON 字符串
    let json_string = serde_json::to_string(&john)?;
    println!("Serialized JSON string: {}", json_string);

    // 将 JSON 字符串反序列化为结构体
    let deserialized_person: User = serde_json::from_str(&json_string)?;
    println!("Deserialized Person: {:?}", deserialized_person);
    Ok(())
}