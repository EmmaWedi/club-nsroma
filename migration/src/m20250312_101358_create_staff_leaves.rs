use sea_orm_migration::prelude::*;

use crate::m20250311_114321_create_staff::Employees;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StaffLeaves::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StaffLeaves::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(StaffLeaves::EmployeeId).uuid().not_null())
                    .col(ColumnDef::new(StaffLeaves::StartDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(StaffLeaves::EndDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(StaffLeaves::LeaveStatus)
                            .string()
                            .check(Expr::col(StaffLeaves::LeaveStatus).is_in(vec![
                                LeaveStatusEnum::Pending.as_str(),
                                LeaveStatusEnum::Approved.as_str(),
                                LeaveStatusEnum::Cancelled.as_str(),
                                LeaveStatusEnum::InSession.as_str(),
                                LeaveStatusEnum::Completed.as_str(),
                            ]))
                            .default(LeaveStatusEnum::Pending.as_str()),
                    )
                    .col(
                        ColumnDef::new(StaffLeaves::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StaffLeaves::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StaffLeaves::Table, StaffLeaves::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffLeaves::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffLeaves {
    Table,
    Id,
    EmployeeId,
    StartDate,
    EndDate,
    LeaveStatus,
    CreatedAt,
    UpdatedAt,
}

enum LeaveStatusEnum {
    Pending,
    Approved,
    Completed,
    InSession,
    Cancelled,
}

impl LeaveStatusEnum {
    fn as_str(&self) -> &'static str {
        match self {
            LeaveStatusEnum::Pending => "PENDING",
            LeaveStatusEnum::Approved => "APPROVED",
            LeaveStatusEnum::Completed => "COMPLETED",
            LeaveStatusEnum::InSession => "INSESSION",
            LeaveStatusEnum::Cancelled => "CANCELLED",
        }
    }
}
