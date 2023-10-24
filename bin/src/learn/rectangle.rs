
    #[derive(Debug)]
    pub struct Rectangle {
        pub(crate) width: u32,
        pub(crate) length: u32,
    }

    impl Rectangle {//绑定方法到 struct上   impl  结构体名 { 对应的方法 }
    fn area(&self) -> u32 {//也有可变与不可变 self  &self  &mut self  对应 获得所有权  借用 可变借用
        self.width * self.length
    }

        // 关联函数，是函数不是方法 通过用于构造器
//调用关联函数使用  类型名：：函数名
        fn square(size:u32) -> Rectangle {//创建一个正方形
            Rectangle {
                width: size,
                length: size,
            }
        }
    }
