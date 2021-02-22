struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("youngjuning"),
        email: String::from("youngjuning@aliyun.com"),
        sign_in_count: 1,
        active: true,
    };
    let user2 = User {
        username: String::from("luozhu"),
        email: String::from("luozhu@163.com"),
        ..user1
    };
    println!("Hello, {}!", user1.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
