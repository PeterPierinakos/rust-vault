use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteUserSecretForm {
	pub secret_key: String,
	pub target_username: String,
}
