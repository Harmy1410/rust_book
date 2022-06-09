#[warn(dead_code)]
struct User {
    username: String,
    email: String,
    active: bool,
    user_called: u16,
}

struct Color(i32, i32, i32);
struct AnotherStruct;

fn main() {
    //      ────────────────────────────────────────────────────────────
    let mut user1 = User {
        username: String::from("1111"),
        email: String::from("1111@22.com"),
        active: true,
        user_called: 32,
    };
    user1.email = String::from("1111@222.33");
    tp(&user1);

    //      ────────────────────────────────────────────────────────────
    let mut user2 = new_user(String::from("1110"), String::from("1110@222.33"));
    user2.username = String::from("1101");
    user2.email = String::from("1101@222.33");
    tp(&user2);

    //      ────────────────────────────────────────────────────────────
    let mut user2 = User {
        username: String::from("0111"),
        ..user1
    };
    user2.email = String::from("1101@222.33");

    user1.user_called += 1;
    user1.active = false;
    // tp(&user1);
    tp(&user2);

    //      ────────────────────────────────────────────────────────────
    let color = Color(0, 0, 0);
    println!("{}", color.0);

    //      ────────────────────────────────────────────────────────────
    let a = AnotherStruct;
}

fn new_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        user_called: 1,
    }
}

fn tp(user: &User) {
    println!(
        "\n{},\t{},\t{},\t{}\n",
        user.username, user.email, user.active, user.user_called
    );
}
