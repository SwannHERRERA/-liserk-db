use validator::Validate;

use crate::{core::Error, database::CONNECTION};

pub async fn create<T: Send + Validate>(mut model: T) -> Result<T, Error> {
    let connection = CONNECTION.get().await;
    T::find_one(connection)
}
