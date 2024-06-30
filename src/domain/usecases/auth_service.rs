pub trait AuthService {
    fn new() -> Self;
    fn generate_token(&self, user_uuid: &str) -> String;
    fn validate_token(&self, token: &str) -> bool;
}
