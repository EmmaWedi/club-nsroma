use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedules::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Schedules::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Schedules::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Schedules::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Schedules::Name).string().not_null())
                    .col(ColumnDef::new(Schedules::StartDate).date())
                    .col(ColumnDef::new(Schedules::EndDate).date())
                    .col(ColumnDef::new(Schedules::StartTime).time())
                    .col(ColumnDef::new(Schedules::EndTime).time())
                    .col(ColumnDef::new(Schedules::Description).string())
                    .col(
                        ColumnDef::new(Schedules::IsCancelled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Schedules::IsStudentEvent)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Schedules::ImageId).string())
                    .col(ColumnDef::new(Schedules::Fee).decimal().not_null())
                    .col(
                        ColumnDef::new(Schedules::IsDiscounted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Schedules::IsFreebie)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Schedules::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Schedules::IsRecurring)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Schedules::RecurringType)
                            .string()
                            .not_null()
                            .check(Expr::col(Schedules::RecurringType).is_in(vec![
                                RecurringTypeEnum::Daily.as_str(),
                                RecurringTypeEnum::Weekly.as_str(),
                                RecurringTypeEnum::Monthly.as_str(),
                                RecurringTypeEnum::Yearly.as_str(),
                            ]))
                            .default(RecurringTypeEnum::Daily.as_str()),
                    )
                    .col(
                        ColumnDef::new(Schedules::MinAgeLimit)
                            .integer()
                            .default(18)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Schedules::MaxAgeLimit)
                            .integer()
                            .default(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Schedules::DiscountRate).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Schedules::IsActive).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(Schedules::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Schedules::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Schedules::Table, Schedules::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Schedules::Table, Schedules::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Schedules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Schedules {
    Table,
    Id,
    OrganizationId,
    BranchId,
    Name,
    StartDate,
    EndDate,
    StartTime,
    EndTime,
    Description,
    IsCancelled,
    IsStudentEvent,
    ImageId,
    Fee,
    IsDiscounted,
    IsFreebie,
    IsDeleted,
    IsRecurring,
    IsActive,
    RecurringType,
    DiscountRate,
    MinAgeLimit,
    MaxAgeLimit,
    CreatedAt,
    UpdatedAt,
}

enum RecurringTypeEnum {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl RecurringTypeEnum {
    fn as_str(&self) -> &str {
        match self {
            RecurringTypeEnum::Daily => "DAILY",
            RecurringTypeEnum::Weekly => "WEEKLY",
            RecurringTypeEnum::Monthly => "MONTHLY",
            RecurringTypeEnum::Yearly => "YEARLY",
        }
    }
}
