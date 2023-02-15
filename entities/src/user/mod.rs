pub mod database_helpers;
pub mod permissions;

use crate::user::permissions::UserPermissions;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[sea_orm(column_type = "Json")]
    pub permissions: UserPermissions,
    #[serde(skip_serializing)]
    pub password: String,
    pub created: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::auth_token::Entity")]
    AuthToken,
}

impl Related<crate::auth_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthToken.def()
    }
}
