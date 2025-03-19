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
                    .table(Suppliers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Suppliers::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Suppliers::Name).string().not_null())
                    .col(ColumnDef::new(Suppliers::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Suppliers::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Suppliers::Contact).string().not_null())
                    .col(ColumnDef::new(Suppliers::Email).string().not_null())
                    .col(
                        ColumnDef::new(Suppliers::DeliveryDates)
                            .timestamp_with_time_zone()
                            .not_null()
                            .check(
                                Expr::col(Suppliers::DeliveryDates).lte(Expr::cust("CURRENT_DATE")),
                            ),
                    )
                    .col(
                        ColumnDef::new(Suppliers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Suppliers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Suppliers::Table, Suppliers::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Suppliers::Table, Suppliers::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Suppliers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Suppliers {
    Table,
    Id,
    Name,
    OrganizationId,
    BranchId,
    Contact,
    Email,
    DeliveryDates,
    CreatedAt,
    UpdatedAt,
}
