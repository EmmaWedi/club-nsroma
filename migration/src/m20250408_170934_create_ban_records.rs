use sea_orm_migration::prelude::*;

use crate::m20250311_135726_create_customers::Customers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BanRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BanRecords::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(BanRecords::BannedReason).string())
                    .col(ColumnDef::new(BanRecords::BannedDuration).integer().default(0))
                    .col(
                        ColumnDef::new(BanRecords::BannedAt)
                            .timestamp_with_time_zone(),
                    )
                    .col(ColumnDef::new(BanRecords::BannedUntil).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(BanRecords::BannedDurationType)
                            .string()
                            .check(Expr::col(BanRecords::BannedDurationType).is_in(vec![
                                BannedDurationType::Days.as_str(),
                                BannedDurationType::Weeks.as_str(),
                                BannedDurationType::Months.as_str(),
                            ])),
                    )
                    .col(ColumnDef::new(BanRecords::CustomerId).uuid())
                    .col(ColumnDef::new(BanRecords::IsActive).boolean().default(false).not_null())
                    .col(
                        ColumnDef::new(BanRecords::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BanRecords::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BanRecords::Table, BanRecords::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BanRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BanRecords {
    Table,
    Id,
    CustomerId,
    BannedReason,
    BannedDuration,
    BannedDurationType,
    BannedAt,
    BannedUntil,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

enum BannedDurationType {
    Months,
    Days,
    Weeks,
}

impl BannedDurationType {
    pub fn as_str(&self) -> &str {
        match self {
            BannedDurationType::Days => "DAYS",
            BannedDurationType::Months => "MONTHS",
            BannedDurationType::Weeks => "WEEKS",
        }
    }
}
