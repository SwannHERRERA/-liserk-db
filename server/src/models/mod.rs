pub mod user;

use crate::core::Error;

pub async fn sync_indexes() -> Result<(), Error> {
    user::User::sync_indexes().await?;

    Ok(())
}
