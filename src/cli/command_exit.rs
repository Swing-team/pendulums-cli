pub enum CommandExit {
  #[allow(dead_code)]
  Normal(String),
  Success(String),
  Error(String),
}
