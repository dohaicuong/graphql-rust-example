use sea_orm::{DbConn, EntityTrait, Schema, ConnectionTrait};

use crate::graphql::post;
pub async fn migrate(db: &DbConn) {
    create_table(db, post::model::Entity).await;
}

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(statement).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}
