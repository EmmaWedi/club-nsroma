use sea_orm_migration::prelude::*;

use crate::{m20250311_100140_create_countries::Countries, m20250311_102524_create_organizations::Organizations};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Branches::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Branches::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Branches::Name).string().not_null())
                    .col(ColumnDef::new(Branches::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Branches::Location).string())
                    .col(ColumnDef::new(Branches::PostCode).string())
                    .col(
                        ColumnDef::new(Branches::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Branches::Contact).string())
                    .col(ColumnDef::new(Branches::Email).string())
                    .col(
                        ColumnDef::new(Branches::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Branches::CountryId).uuid().not_null())
                    .col(
                        ColumnDef::new(Branches::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Branches::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Branches::Table, Branches::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Branches::Table, Branches::CountryId)
                            .to(Countries::Table, Countries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Branches::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Branches {
    Table,
    Id,
    Name,
    OrganizationId,
    Location,
    PostCode,
    IsActive,
    Contact,
    Email,
    IsDeleted,
    CountryId,
    CreatedAt,
    UpdatedAt,
}
