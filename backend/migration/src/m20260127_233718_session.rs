use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("sessions")
                    .if_not_exists()
                    .col(uuid("id"))
                    .col(uuid("tiktok_user_id"))
                    .col(string("refresh_token"))
                    .col(timestamp("expires_at").timestamp().not_null())
                    .col(boolean("revoked").default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("sessions").to_owned())
            .await
    }
}
