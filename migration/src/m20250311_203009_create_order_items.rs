use sea_orm_migration::prelude::*;

use crate::{
    m20250311_132020_create_stock_drinks::StockDrinks,
    m20250311_133559_create_stock_foods::StockFoods, m20250311_183402_create_orders::Orders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(OrderItems::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::StockDrinkId).uuid())
                    .col(ColumnDef::new(OrderItems::StockFoodId).uuid())
                    .col(
                        ColumnDef::new(OrderItems::ItemType)
                            .string()
                            .not_null()
                            .check(Expr::col(OrderItems::ItemType).is_in(vec![
                                ItemTypeEnum::Drink.as_str(),
                                ItemTypeEnum::Food.as_str(),
                            ]))
                            .default(ItemTypeEnum::Drink.as_str()),
                    )
                    .col(
                        ColumnDef::new(OrderItems::Price)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::Quantity)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrderItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::StockDrinkId)
                            .to(StockDrinks::Table, StockDrinks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::StockFoodId)
                            .to(StockFoods::Table, StockFoods::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    StockDrinkId,
    StockFoodId,
    ItemType,
    Price,
    Quantity,
    CreatedAt,
    UpdatedAt,
}

enum ItemTypeEnum {
    Food,
    Drink,
}

impl ItemTypeEnum {
    fn as_str(&self) -> &'static str {
        match self {
            ItemTypeEnum::Drink => "DRINK",
            ItemTypeEnum::Food => "FOOD",
        }
    }
}
