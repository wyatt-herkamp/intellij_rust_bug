use crate::{auth_token, AuthTokenEntity, AuthTokenModel, UserEntity, UserModel};
use sea_orm::prelude::Uuid;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, JoinType, QuerySelect, QueryTrait};

#[inline(always)]
pub async fn find_by_id(
    connections: &impl ConnectionTrait,
    id: Uuid,
) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find_by_id
        (id)
        .one(connections)
        .await
}
