const EMAIL_VERIFICATION: &str =
  r"^[a-zA-Z0-9+_%-+.]{1,256}@[a-zA-Z0-9][a-zA-Z0-9-]{0,64}(.[a-zA-Z0-9][a-zA-Z0-9-]{0,25})+$";

pub mod sign_in;
pub mod sign_up;
