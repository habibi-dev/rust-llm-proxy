use crate::features::users::model::{api_key, user};
use crate::features::users::utility::hash_key::hash_key;
use crate::features::users::utility::key_generator::key_generator;
use sea_orm::PaginatorTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn run(db: &DatabaseConnection) -> anyhow::Result<()> {
    let count = user::Entity::find().count(db).await?;
    if count > 0 {
        println!("ℹ️  Users already exist, skipping seeder.");
        return Ok(());
    }

    let name = "admin";

    let raw_key: String = key_generator();

    let key_hash = hash_key(&raw_key);

    let admin = user::ActiveModel {
        name: Set(name.into()),
        is_admin: Set(true),
        status: Set(true),
        ..Default::default()
    };
    let admin = admin.insert(db).await?;

    // Generate API key
    let api_key = api_key::ActiveModel {
        user_id: Set(admin.id),
        key_hash: Set(key_hash),
        status: Set(true),
        ..Default::default()
    };

    api_key.insert(db).await?;

    println!("✅ Admin user seeded (name: {name})");
    println!("✅ Api key: {}", raw_key);
    Ok(())
}
