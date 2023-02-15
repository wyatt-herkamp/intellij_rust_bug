use crate::{auth_token, AuthTokenEntity, AuthTokenModel, UserEntity, UserModel};
use sea_orm::prelude::Uuid;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseBackend, JoinType, QuerySelect, QueryTrait};

#[inline(always)]
pub async fn find_by_id(
    connections: &impl ConnectionTrait,
    id: Uuid,
) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find()
        .filter(crate::user::Column::Id.eq(id))
        .one(connections)
        .await
}
/// Uses a Join to get the AuthTokenModel and UserModel
pub async fn get_user_and_auth_token_from_token(
    connections: &impl ConnectionTrait,
    token: &str,
) -> Result<Option<(AuthTokenModel, UserModel)>, DbErr> {
    AuthTokenEntity::find()
        .find_also_related(UserEntity)
        .filter(
            auth_token::Column::TokenHash
                .eq(token)
                .and(auth_token::Column::Revoked.eq(false)),
        )
        .one(connections)
        .await
        .map(|result| {
            // Map the result to a tuple of (AuthTokenModel, UserModel)
            if let Some((token, user)) = result {
                if let Some(user) = user {
                    Some((token, user))
                } else {
                    None
                }
            } else {
                None
            }
        })
}
#[inline(always)]
pub async fn first_user(connections: &impl ConnectionTrait) -> Result<bool, DbErr> {
    UserEntity::find()
        .count(connections)
        .await
        .map(|count| count == 0)
}
/// Checks if a user exists by checking if the username or email is already in use
pub async fn does_user_exist(
    connections: &impl ConnectionTrait,
    username: &str,
    email: &str,
) -> Result<bool, DbErr> {
    UserEntity::find()
        .filter(
            crate::user::Column::Username
                .eq(username)
                .or(crate::user::Column::Email.eq(email)),
        )
        .count(connections)
        .await
        .map(|count| count > 0)
}
