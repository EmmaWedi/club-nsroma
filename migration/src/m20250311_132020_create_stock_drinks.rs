use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_134128_create_category::Categories,
    m20250311_134626_create_suppliers::Suppliers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockDrinks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockDrinks::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(StockDrinks::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockDrinks::BranchId).uuid().not_null())
                    .col(ColumnDef::new(StockDrinks::Name).string().not_null())
                    .col(ColumnDef::new(StockDrinks::UnitPrice).decimal().default(0.0).not_null())
                    .col(
                        ColumnDef::new(StockDrinks::IsAlcholic)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(StockDrinks::IsCustom)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(StockDrinks::Quantity).integer().default(0).not_null())
                    .col(
                        ColumnDef::new(StockDrinks::QuantitySold)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockDrinks::SalePrice).decimal().not_null())
                    .col(
                        ColumnDef::new(StockDrinks::IsMarkupApplied)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(StockDrinks::MarkupRate).decimal().not_null())
                    .col(
                        ColumnDef::new(StockDrinks::IsReduced)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(StockDrinks::ReductionRate)
                            .decimal()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockDrinks::ImageUrl).string())
                    .col(
                        ColumnDef::new(StockDrinks::StockStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(StockDrinks::StockStatus).is_in(vec![
                                StockStatusEnum::InStock.as_str(),
                                StockStatusEnum::OutOfStock.as_str(),
                                StockStatusEnum::RunningLow.as_str(),
                            ]))
                            .default(StockStatusEnum::OutOfStock.as_str()),
                    )
                    .col(ColumnDef::new(StockDrinks::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(StockDrinks::SupplierId).uuid().not_null())
                    .col(
                        ColumnDef::new(StockDrinks::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StockDrinks::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockDrinks::Table, StockDrinks::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockDrinks::Table, StockDrinks::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockDrinks::Table, StockDrinks::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockDrinks::Table, StockDrinks::SupplierId)
                            .to(Suppliers::Table, Suppliers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockDrinks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StockDrinks {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    UnitPrice,
    IsAlcholic,
    IsCustom,
    Quantity,
    QuantitySold,
    SalePrice,
    IsMarkupApplied,
    MarkupRate,
    IsReduced,
    ReductionRate,
    ImageUrl,
    StockStatus,
    CategoryId,
    SupplierId,
    CreatedAt,
    UpdatedAt,
}

enum StockStatusEnum {
    InStock,
    OutOfStock,
    RunningLow,
}

impl StockStatusEnum {
    fn as_str(&self) -> &str {
        match self {
            StockStatusEnum::InStock => "in_stock",
            StockStatusEnum::OutOfStock => "out_of_stock",
            StockStatusEnum::RunningLow => "running_low",
        }
    }
}
