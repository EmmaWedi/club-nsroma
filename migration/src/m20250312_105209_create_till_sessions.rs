use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_114321_create_staff::Employees,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TillSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TillSessions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(TillSessions::EmployeeId).uuid().not_null())
                    .col(ColumnDef::new(TillSessions::BranchId).uuid().not_null())
                    .col(
                        ColumnDef::new(TillSessions::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TillSessions::StartTime).time())
                    .col(ColumnDef::new(TillSessions::EndTime).time())
                    .col(
                        ColumnDef::new(TillSessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TillSessions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TillSessions::Table, TillSessions::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TillSessions::Table, TillSessions::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TillSessions::Table, TillSessions::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TillSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TillSessions {
    Table,
    Id,
    EmployeeId,
    BranchId,
    OrganizationId,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
