use sea_orm::{DbBackend, MockDatabase};
use sea_orm::prelude::*;


#[tokio::test]
pub async fn test(){
    let mock = MockDatabase::new(DbBackend::Postgres).into_connection();
    entities::UserEntity::find_by_id(Uuid::new_v4()).one(&mock).await;
}

