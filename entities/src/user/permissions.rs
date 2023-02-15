use sea_orm::sea_query::{ArrayType, ColumnType, ValueType, ValueTypeErr};
use sea_orm::{
    ColIdx, DbErr, IntoActiveValue, JsonValue, QueryResult, TryGetError, TryGetable, Value,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPermissions {
    pub create_shares: bool,
    pub create_images: bool,
    pub create_pastes: bool,
    pub admin: bool,
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            create_shares: true,
            create_images: true,
            create_pastes: true,
            admin: false,
        }
    }
}
impl UserPermissions {
    pub fn new_admin() -> Self {
        Self {
            create_shares: true,
            create_images: true,
            create_pastes: true,
            admin: true,
        }
    }
}
impl From<UserPermissions> for JsonValue {
    fn from(value: UserPermissions) -> Self {
        serde_json::to_value(value).unwrap()
    }
}
impl From<UserPermissions> for sea_orm::Value {
    fn from(value: UserPermissions) -> Self {
        sea_orm::Value::Json(Some(serde_json::to_value(value).unwrap().into()))
    }
}
impl TryGetable for UserPermissions {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let val: JsonValue = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        serde_json::from_value(val).map_err(|e| TryGetError::DbErr(DbErr::Json(e.to_string())))
    }
}
impl ValueType for UserPermissions {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(x)) => {
                let auth_properties: UserPermissions =
                    serde_json::from_value(*x).map_err(|_error| ValueTypeErr)?;
                Ok(auth_properties)
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(UserPermissions).to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::Json
    }

    fn column_type() -> ColumnType {
        ColumnType::Json
    }
}
impl IntoActiveValue<UserPermissions> for UserPermissions {
    fn into_active_value(self) -> sea_orm::ActiveValue<UserPermissions> {
        sea_orm::ActiveValue::Set(self)
    }
}
