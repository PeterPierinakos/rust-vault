use actix_session::Session;

pub fn verify_session(session: &Session) -> bool {
    if let Some(_id) = session.get::<i32>("id").unwrap() {
        return true;
    } else {
        return false;
    }
}

pub fn is_valid_auth_string(s: &String) -> bool {
    for c in s.chars() {
         if c == ' ' { return false };
    }
    true
}
