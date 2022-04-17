// Global variables for configuration listed here

pub const ADRESS: [i32; 4] = [127, 0, 0, 1];
pub const PORT: i32 = 8080;
pub const REDIS_ADRESS: [i32; 4] = [127, 0, 0, 1];
pub const REDIS_PORT: i32 = 6379;
pub const DATABASE_URL: &str = "postgres://postgres:0@127.0.0.1/rustvault";
// These keys are very secret, don't give to anyone
pub const VERY_SECRET_PRIVATE_KEY: &str = "1234";