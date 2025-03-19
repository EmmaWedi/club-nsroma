use sea_orm_migration::prelude::*;

use crate::m20250311_100140_create_countries::Countries;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organizations::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Organizations::Name).string().not_null())
                    .col(ColumnDef::new(Organizations::LogoUrl).string())
                    .col(
                        ColumnDef::new(Organizations::RegisteredAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .check(Expr::col(Organizations::RegisteredAt).lte(Expr::cust("CURRENT_DATE"))),
                    )
                    .col(
                        ColumnDef::new(Organizations::IsBlocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Organizations::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Organizations::WebUrl).string())
                    .col(ColumnDef::new(Organizations::Location).string())
                    .col(ColumnDef::new(Organizations::PostCode).string())
                    .col(ColumnDef::new(Organizations::CountryId).uuid().not_null())
                    .col(
                        ColumnDef::new(Organizations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Organizations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Organizations::Table, Organizations::CountryId)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Organizations {
    Table,
    Id,
    Name,
    LogoUrl,
    RegisteredAt,
    IsBlocked,
    IsActive,
    WebUrl,
    Location,
    PostCode,
    CountryId,
    CreatedAt,
    UpdatedAt,
}
