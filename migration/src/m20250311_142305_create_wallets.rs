use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_135726_create_customers::Customers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wallets::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Wallets::CustomerId).uuid())
                    .col(ColumnDef::new(Wallets::WalletNumber).string().not_null())
                    .col(ColumnDef::new(Wallets::Currency).string().not_null())
                    .col(
                        ColumnDef::new(Wallets::WalletStatus)
                            .string()
                            .check(Expr::col(Wallets::WalletStatus).is_in(vec![
                                WalletStatusEnum::Active.as_str(),
                                WalletStatusEnum::Inactive.as_str(),
                                WalletStatusEnum::Suspended.as_str(),
                                WalletStatusEnum::Closed.as_str(),
                            ]))
                            .default(WalletStatusEnum::Active.as_str()),
                    )
                    .col(
                        ColumnDef::new(Wallets::Balance)
                            .decimal()
                            .default(0.0)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Wallets::IsDepositBlocked)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Wallets::IsWithdrawalBlocked)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Wallets::OwnerType)
                            .string()
                            .check(Expr::col(Wallets::OwnerType).is_in(vec![
                                OwnerTypeEnum::Customer.as_str(),
                                OwnerTypeEnum::Organization.as_str(),
                                OwnerTypeEnum::Branch.as_str(),
                            ]))
                            .default(OwnerTypeEnum::Customer.as_str()),
                    )
                    .col(ColumnDef::new(Wallets::OrganizationId).uuid())
                    .col(ColumnDef::new(Wallets::BranchId).uuid())
                    .col(
                        ColumnDef::new(Wallets::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Wallets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Wallets::Table, Wallets::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Wallets::Table, Wallets::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Wallets::Table, Wallets::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Wallets {
    Table,
    Id,
    CustomerId,
    WalletNumber,
    Currency,
    WalletStatus,
    Balance,
    IsDepositBlocked,
    IsWithdrawalBlocked,
    OwnerType,
    OrganizationId,
    BranchId,
    CreatedAt,
    UpdatedAt,
}

enum WalletStatusEnum {
    Active,
    Inactive,
    Suspended,
    Closed,
}

impl WalletStatusEnum {
    fn as_str(&self) -> &str {
        match self {
            WalletStatusEnum::Active => "active",
            WalletStatusEnum::Inactive => "inactive",
            WalletStatusEnum::Suspended => "suspended",
            WalletStatusEnum::Closed => "closed",
        }
    }
}

enum OwnerTypeEnum {
    Customer,
    Organization,
    Branch,
}

impl OwnerTypeEnum {
    fn as_str(&self) -> &str {
        match self {
            OwnerTypeEnum::Customer => "customer",
            OwnerTypeEnum::Organization => "organization",
            OwnerTypeEnum::Branch => "branch",
        }
    }
}
