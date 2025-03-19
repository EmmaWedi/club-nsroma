use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_123539_create_schedule::Schedules,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Freebies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Freebies::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Freebies::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Freebies::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Freebies::ScheduleId).uuid().not_null())
                    .col(ColumnDef::new(Freebies::Validity).date().not_null())
                    .col(ColumnDef::new(Freebies::TimeUntil).time().not_null())
                    .col(
                        ColumnDef::new(Freebies::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Freebies::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Freebies::Table, Freebies::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Freebies::Table, Freebies::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Freebies::Table, Freebies::ScheduleId)
                            .to(Schedules::Table, Schedules::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Freebies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Freebies {
    Table,
    Id,
    OrganizationId,
    BranchId,
    ScheduleId,
    Validity,
    TimeUntil,
    CreatedAt,
    UpdatedAt,
}
