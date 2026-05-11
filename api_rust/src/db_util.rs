use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::entity::user;
use crate::state::AppError;


pub async fn retrieve_user_by_email(db: &DatabaseConnection, email: &str) -> Option<user::Model> {
    user::Entity::find()
        .filter(user::Column::EmailAddr.eq(email))
        .one(db)
        .await
        .ok()
        .flatten()
}

pub async fn user_already_exists(
    db: &DatabaseConnection,
    username: &str,
    email: &str,
) -> Result<bool, AppError> {
    let condition = Conditiosystemctl reload --user app-com.mitchellh.ghostty.service   n::any()
        .add(user::Column::Username.eq(username))
        .add(user::Column::EmailAddr.eq(email));
    let count = user::Entity::find().filter(condition).count(db).await?;

    Ok(count > 0)
}

pub async fn add_new_user(
    db: &DatabaseConnection,
    new_user: user::ActiveModel,
) -> Result<(), AppError> {
    let _ans = user::Entity::insert(new_user).exec(db).await?;
    Ok(())
}
