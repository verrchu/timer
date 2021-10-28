use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
}

mod transform {
    use super::User;
    use crate::domain;

    impl From<domain::user::User> for User {
        fn from(input: domain::user::User) -> Self {
            Self {
                user_id: input.user_id.0,
            }
        }
    }
}
