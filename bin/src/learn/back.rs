use std::collections::HashMap;
//use crate::libe::learn01::rectangle;

// use crate::learn;

// use self::learn;

mod learn;

mod libe;

fn back_main() {

    libe::make_friend();

    let mut v: Vec<i32> = Vec::new();
    v.insert(0,4);

    learn::add_to_waitlist();

    let mut  map = HashMap::new();
    map.insert("key","value");
    println!("{:?}",map);

    println!("234");
    test();
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    let y = x;
    println!("{},{}",x,y);

    let mut  str = String::new();
    str.push_str("hhh");
    println!("{}",str);
    let str2 = &mut str;
    *str2 = "fff".to_string();
    println!("{},{}",str2,str2);

    change(str2);
    println!("{},{}",str2,str2);

    let mut k=String::from("hello");
    {//可以大括号分隔作用域
        let s1=&mut k;
        s1.push_str(",word");
    }//到这里s1就不再有效了,因为已经出了作用域了
    println!("{},{}",k,str2);
    let s2=&mut k;
    println!("{}",s2);

    let sq = learn::rectangle::Rectangle{
        width: 10,
        length: 5,
    };

    println!("{:#?},{:?}",sq,sq);


    let sn=Some(5);
    let ss=Some("a string");
    let absent_number: Option<i32>  = None;

    match sn  {
        Some(i32) => println!("ok {:?}",i32),
        None => println!("None")
    }

}

// 可变引用
fn change(some_string: &mut String){
    some_string.push_str(", world");
    //相当拼接字符串的功能 append
}

fn test(){
    println!("rr");
}
