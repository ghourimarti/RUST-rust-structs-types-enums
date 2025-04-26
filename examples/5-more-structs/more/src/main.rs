struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32);

fn main() {
    println!("\n<=====================================>\n");
    let username = String::from("johndoe");
    let email = String::from("john@example.com");
    let uri = String::from("https://example.com");
    let active=  true;

    let user = User { username, email, uri, active };
    let my_point = Point(10, 20, 30);
    println!("points 0: {}", my_point.0);
    println!("points 1: {}", my_point.1);
    println!("points 2: {}", my_point.2);
}
