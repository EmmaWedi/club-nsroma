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
                    .table(Departments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Departments::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Departments::Name).string().not_null())
                    .col(ColumnDef::new(Departments::Description).string())
                    .col(
                        ColumnDef::new(Departments::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Departments::BranchId).uuid().not_null())
                    .col(
                        ColumnDef::new(Departments::IsForAllBranches)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Departments::NumberOfEmployees).integer())
                    .col(ColumnDef::new(Departments::NumberOfAllowedLeaveDays).integer())
                    .col(ColumnDef::new(Departments::DailyRate).decimal().not_null())
                    .col(ColumnDef::new(Departments::HourlyRate).decimal().not_null())
                    .col(
                        ColumnDef::new(Departments::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Departments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Departments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Departments::DeletedAt)
                            .timestamp_with_time_zone()
                            .check(
                                Expr::col(Departments::DeletedAt).lte(Expr::cust("CURRENT_DATE")),
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Departments::Table, Departments::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Departments::Table, Departments::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Departments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Departments {
    Table,
    Id,
    Name,
    Description,
    OrganizationId,
    BranchId,
    IsForAllBranches,
    NumberOfEmployees,
    NumberOfAllowedLeaveDays,
    DailyRate,
    HourlyRate,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
