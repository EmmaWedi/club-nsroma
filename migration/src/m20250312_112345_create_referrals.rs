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
                    .table(Referrals::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Referrals::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Referrals::ReferrerId).uuid().not_null())
                    .col(ColumnDef::new(Referrals::ReferredId).uuid().not_null())
                    .col(
                        ColumnDef::new(Referrals::RewardAmount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Referrals::ReferralStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Referrals::ReferralStatus).is_in(vec![
                                ReferralStatusEnum::Pending.as_str(),
                                ReferralStatusEnum::Paid.as_str(),
                                ReferralStatusEnum::Failed.as_str(),
                            ]))
                            .default(ReferralStatusEnum::Pending.as_str()),
                    )
                    .col(
                        ColumnDef::new(Referrals::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Referrals::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Referrals::Table, Referrals::ReferrerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Referrals::Table, Referrals::ReferredId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Referrals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Referrals {
    Table,
    Id,
    ReferrerId,
    ReferredId,
    RewardAmount,
    ReferralStatus,
    CreatedAt,
    UpdatedAt,
}

enum ReferralStatusEnum {
    Pending,
    Paid,
    Failed,
}

impl ReferralStatusEnum {
    fn as_str(&self) -> &str {
        match self {
            ReferralStatusEnum::Pending => "pending",
            ReferralStatusEnum::Paid => "paid",
            ReferralStatusEnum::Failed => "failed",
        }
    }
}
