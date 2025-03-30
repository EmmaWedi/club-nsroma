use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_134128_create_category::Categories,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockFoods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockFoods::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(StockFoods::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(StockFoods::BranchId).uuid().not_null())
                    .col(ColumnDef::new(StockFoods::Name).string().not_null())
                    .col(ColumnDef::new(StockFoods::CategoryId).uuid().not_null())
                    .col(
                        ColumnDef::new(StockFoods::Price)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockFoods::Description).string())
                    .col(ColumnDef::new(StockFoods::ImageId).string())
                    .col(
                        ColumnDef::new(StockFoods::IsCustom)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(StockFoods::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StockFoods::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockFoods::Table, StockFoods::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockFoods::Table, StockFoods::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockFoods::Table, StockFoods::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockFoods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StockFoods {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    CategoryId,
    Price,
    Description,
    ImageId,
    IsCustom,
    CreatedAt,
    UpdatedAt,
}
