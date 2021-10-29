use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub alias: String,
}

mod transform {
    use super::User;
    use crate::domain;

    impl From<domain::user::User> for User {
        fn from(input: domain::user::User) -> Self {
            Self { alias: input.alias }
        }
    }
}
