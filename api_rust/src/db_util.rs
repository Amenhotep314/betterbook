use crate::entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn retrieve_user_by_email(db: &DatabaseConnection, email: &str) -> Option<user::Model> {
    user::Entity::find()
        .filter(user::Column::EmailAddr.eq(email))
        .one(db)
        .await
        .ok()
        .flatten()
}
