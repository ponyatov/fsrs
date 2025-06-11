pub const PORT: u16 = 12345;
pub const HOST: &str = "127.0.0.1";
pub const BIND: &str = const_format::formatcp!("{}:{}", HOST, PORT);
