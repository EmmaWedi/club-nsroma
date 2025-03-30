use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_135726_create_customers::Customers
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Transactions::CustomerId).uuid())
                    .col(ColumnDef::new(Transactions::OrganizationId).uuid())
                    .col(ColumnDef::new(Transactions::BranchId).uuid())
                    .col(
                        ColumnDef::new(Transactions::Amount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::TransactionType)
                            .string()
                            .not_null()
                            .check(Expr::col(Transactions::TransactionType).is_in(vec![
                                TransactionTypeEnum::TopUp.as_str(),
                                TransactionTypeEnum::Payment.as_str(),
                                TransactionTypeEnum::Withdrawal.as_str(),
                                TransactionTypeEnum::Refund.as_str(),
                                TransactionTypeEnum::Tip.as_str(),
                            ]))
                            .default(TransactionTypeEnum::TopUp.as_str()),
                    )
                    .col(ColumnDef::new(Transactions::DebitChannel).string())
                    .col(ColumnDef::new(Transactions::CreditChannel).string())
                    .col(
                        ColumnDef::new(Transactions::TransactionStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Transactions::TransactionStatus).is_in(vec![
                                TransactionStatusEnum::Pending.as_str(),
                                TransactionStatusEnum::Success.as_str(),
                                TransactionStatusEnum::Failed.as_str(),
                                TransactionStatusEnum::Reversed.as_str(),
                            ]))
                            .default(TransactionStatusEnum::Pending.as_str()),
                    )
                    .col(
                        ColumnDef::new(Transactions::Charges)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Transactions::Description).string())
                    .col(ColumnDef::new(Transactions::TransactionReference).string())
                    .col(ColumnDef::new(Transactions::ExternalReference).string())
                    .col(
                        ColumnDef::new(Transactions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Transactions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transactions::Table, Transactions::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transactions::Table, Transactions::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transactions::Table, Transactions::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    CustomerId,
    OrganizationId,
    BranchId,
    Amount,
    TransactionType,
    DebitChannel,
    CreditChannel,
    TransactionStatus,
    Charges,
    Description,
    TransactionReference,
    ExternalReference,
    CreatedAt,
    UpdatedAt,
}

enum TransactionTypeEnum {
    TopUp,
    Payment,
    Withdrawal,
    Refund,
    Tip,
}

impl TransactionTypeEnum {
    fn as_str(&self) -> &str {
        match self {
            TransactionTypeEnum::TopUp => "TOPUP",
            TransactionTypeEnum::Payment => "PAYMENT",
            TransactionTypeEnum::Withdrawal => "WITHDRAWAL",
            TransactionTypeEnum::Refund => "REFUND",
            TransactionTypeEnum::Tip => "TIP",
        }
    }
}

enum TransactionStatusEnum {
    Pending,
    Success,
    Failed,
    Reversed,
}

impl TransactionStatusEnum {
    fn as_str(&self) -> &str {
        match self {
            TransactionStatusEnum::Pending => "PENDING",
            TransactionStatusEnum::Success => "SUCCESS",
            TransactionStatusEnum::Failed => "FAILED",
            TransactionStatusEnum::Reversed => "REVERSED",
        }
    }
}
