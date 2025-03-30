use sea_orm_migration::prelude::*;

use crate::m20250311_165323_create_transactions::Transactions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Externals::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Externals::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Externals::TransactionId).uuid().not_null())
                    .col(
                        ColumnDef::new(Externals::ExternalStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Externals::ExternalStatus).is_in(vec![
                                ExternalStatusEnum::Pending.as_str(),
                                ExternalStatusEnum::Success.as_str(),
                                ExternalStatusEnum::Failed.as_str(),
                            ]))
                            .default(ExternalStatusEnum::Pending.as_str()),
                    )
                    .col(ColumnDef::new(Externals::ExternalId).string().not_null())
                    .col(ColumnDef::new(Externals::ExternalData).json())
                    .col(ColumnDef::new(Externals::CallBackData).json())
                    .col(ColumnDef::new(Externals::InitialResponseData).json())
                    .col(ColumnDef::new(Externals::FinalResponseData).json())
                    .col(
                        ColumnDef::new(Externals::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Externals::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Externals::Table, Externals::TransactionId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Externals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Externals {
    Table,
    Id,
    TransactionId,
    ExternalStatus,
    ExternalId,
    ExternalData,
    CallBackData,
    InitialResponseData,
    FinalResponseData,
    CreatedAt,
    UpdatedAt,
}

enum ExternalStatusEnum {
    Pending,
    Success,
    Failed,
}

impl ExternalStatusEnum {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "PENDING",
            Self::Success => "SUCCESS",
            Self::Failed => "FAILED",
        }
    }
}
