struct User {
  _username: String,
  _email: String,
  _sign_in_count: u64,
  _active: bool,
}

fn main() {
  let _user1 = build_user(
    String::from("someone@example.com"),
    String::from("someusername123"),
  );
}

fn build_user(username: String, email: String) -> User {
  User {
    _email: email,
    _username: username,
    _active: true,
    _sign_in_count: 1,
  }
}
