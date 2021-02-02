struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let user1 = build_user(
    String::from("someone@example.com"),
    String::from("someusername123"),
  );
}

fn build_user(username: String, email: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
