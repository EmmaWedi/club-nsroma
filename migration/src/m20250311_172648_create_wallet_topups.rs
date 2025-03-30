use sea_orm_migration::prelude::*;

use crate::{
    m20250311_142305_create_wallets::Wallets, m20250311_165323_create_transactions::Transactions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WalletTopups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WalletTopups::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(WalletTopups::WalletId).uuid().not_null())
                    .col(
                        ColumnDef::new(WalletTopups::TransactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WalletTopups::Amount).decimal().not_null())
                    .col(
                        ColumnDef::new(WalletTopups::Status)
                            .string()
                            .check(Expr::col(WalletTopups::Status).is_in(vec![
                                StatusEnum::Success.as_str(),
                                StatusEnum::Failed.as_str(),
                            ]))
                            .default(StatusEnum::Success.as_str()),
                    )
                    .col(
                        ColumnDef::new(WalletTopups::LastBalance)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WalletTopups::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(WalletTopups::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WalletTopups::Table, WalletTopups::WalletId)
                            .to(Wallets::Table, Wallets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WalletTopups::Table, WalletTopups::TransactionId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletTopups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WalletTopups {
    Table,
    Id,
    WalletId,
    TransactionId,
    Amount,
    Status,
    LastBalance,
    CreatedAt,
    UpdatedAt,
}

enum StatusEnum {
    Success,
    Failed,
}

impl StatusEnum {
    fn as_str(&self) -> &str {
        match self {
            StatusEnum::Success => "SUCCESS",
            StatusEnum::Failed => "FAILED",
        }
    }
}
