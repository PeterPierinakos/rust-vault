use actix_session::Session;

pub fn verify_session(session: &Session) -> bool {
    let id = session.get::<i32>("id");
    match id {
        Ok(_id) => {
            return true;
        }
        Err(_err) => {}
    }
    false
}
