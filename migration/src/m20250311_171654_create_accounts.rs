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
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Accounts::CustomerId).uuid().not_null())
                    .col(ColumnDef::new(Accounts::AccountName).string().not_null())
                    .col(ColumnDef::new(Accounts::AccountNumber).string().not_null())
                    .col(
                        ColumnDef::new(Accounts::AccountType)
                            .string()
                            .check(Expr::col(Accounts::AccountType).is_in(vec![
                                AccountTypeEnum::Mobile.as_str(),
                                AccountTypeEnum::Bank.as_str(),
                                AccountTypeEnum::Card.as_str(),
                            ]))
                            .default(AccountTypeEnum::Mobile.as_str()),
                    )
                    .col(ColumnDef::new(Accounts::AccountIssuer).string().not_null())
                    .col(ColumnDef::new(Accounts::IsActive).boolean().default(true))
                    .col(ColumnDef::new(Accounts::IsBlocked).boolean().default(false))
                    .col(ColumnDef::new(Accounts::IsDeleted).boolean().default(false))
                    .col(
                        ColumnDef::new(Accounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Accounts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Accounts::DeletedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Accounts::Table, Accounts::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
    CustomerId,
    AccountName,
    AccountNumber,
    AccountType,
    AccountIssuer,
    IsActive,
    IsBlocked,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

enum AccountTypeEnum {
    Mobile,
    Bank,
    Card,
}

impl AccountTypeEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AccountTypeEnum::Mobile => "mobile",
            AccountTypeEnum::Bank => "bank",
            AccountTypeEnum::Card => "card",
        }
    }
}
