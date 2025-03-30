use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Carousels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Carousels::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Carousels::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Carousels::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Carousels::Name).string().not_null())
                    .col(ColumnDef::new(Carousels::ImageId).string())
                    .col(ColumnDef::new(Carousels::Description).string())
                    .col(
                        ColumnDef::new(Carousels::StartDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .check(Expr::col(Carousels::StartDate).lte(Expr::cust("CURRENT_DATE"))),
                    )
                    .col(ColumnDef::new(Carousels::EndDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Carousels::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Carousels::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Carousels::Table, Carousels::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Carousels::Table, Carousels::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Carousels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Carousels {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    ImageId,
    Description,
    StartDate,
    EndDate,
    CreatedAt,
    UpdatedAt,
}
