mod oneshot;
pub use oneshot::OneshotMessage;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(pub Uuid);

impl From<Uuid> for MessageId {
    fn from(input: Uuid) -> Self {
        Self(input)
    }
}
