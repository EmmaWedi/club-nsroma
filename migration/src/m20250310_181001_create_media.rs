use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS pgcrypto;".to_string(),
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Medias::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Medias::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Medias::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Medias::FileName).string())
                    .col(ColumnDef::new(Medias::FilePath).string())
                    .col(ColumnDef::new(Medias::MimeType).string())
                    .col(ColumnDef::new(Medias::FileSize).big_integer())
                    .col(ColumnDef::new(Medias::MediaType).string())
                    .col(ColumnDef::new(Medias::Width).integer())
                    .col(ColumnDef::new(Medias::Height).integer())
                    .col(ColumnDef::new(Medias::Duration).integer())
                    .col(
                        ColumnDef::new(Medias::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Medias::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Medias::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .get_connection()
        .execute(Statement::from_string(
            manager.get_database_backend(),
            "DROP EXTENSION IF EXISTS pgcrypto;".to_string(),
        ))
        .await?;

        manager
            .drop_table(Table::drop().table(Medias::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Medias {
    Table,
    Id,
    OwnerId,
    FileName,
    FilePath,
    MimeType,
    FileSize,
    MediaType,
    Width,
    Height,
    Duration,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
