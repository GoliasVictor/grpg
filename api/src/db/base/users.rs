use crate::db::base::{
    Store
};
use sea_orm::{Set, EntityTrait};
use crate::db::base::entities::{
    user,
    prelude::User
};
use crate::db::models::UserData;


pub async fn add_user(store: &Store, name: String) -> i32 {
    let db = store.reldb_conn();
    let new_user = user::ActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    };
    let res = User::insert(new_user).exec(db).await.unwrap();

    res.last_insert_id
}

pub async fn get_users(store: &Store) -> Option<Vec<UserData>> {
    let db = store.reldb_conn();

    let users = User::find()
        .all(db)
        .await
        .unwrap();
    Some(users.into_iter().map(|u| UserData {
        id: u.id,
        name: u.name,
    }).collect())
}

pub async fn get_user(store: &Store, user_id: i32) -> Option<UserData> {
    let db = store.reldb_conn();
    let user = User::find_by_id(user_id)
        .one(db)
        .await
        .unwrap();
    user.map(|u| UserData {
        id: u.id,
        name: u.name,
    })
}
