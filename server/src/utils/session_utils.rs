use actix_session::Session;

pub fn verify_session(session: &Session) -> bool {
    if let Some(id) = session.get::<i32>("id").unwrap() {
        return true;
    } else {
        return false;
    }
}
