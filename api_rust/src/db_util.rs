use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

use crate::auth;
use crate::entity::user;

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
) -> Result<bool, DbErr> {
    let condition = Condition::any()
        .add(user::Column::Username.eq(username))
        .add(user::Column::EmailAddr.eq(email));
    let count = user::Entity::find().filter(condition).count(db).await?;

    Ok(count > 0)
}

pub async fn add_new_user(
    db: &DatabaseConnection,
    new_user: user::ActiveModel,
) -> Result<(), DbErr> {
    let ans = user::Entity::insert(new_user).exec(db).await?;
    Ok(())
}
