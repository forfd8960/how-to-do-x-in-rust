use jwt_auth::user;

fn main() {
    println!(
        "{:?}",
        user::User::new("test".to_string(), "test@example.com".to_string())
    );
}
