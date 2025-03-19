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
                    .table(StaffShifts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StaffShifts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(StaffShifts::ClockInTime).time().not_null())
                    .col(ColumnDef::new(StaffShifts::ClockOutTime).time().not_null())
                    .col(ColumnDef::new(StaffShifts::EmployeeId).uuid().not_null())
                    .col(
                        ColumnDef::new(StaffShifts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(StaffShifts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StaffShifts::Table, StaffShifts::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffShifts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StaffShifts {
    Table,
    Id,
    ClockInTime,
    ClockOutTime,
    EmployeeId,
    CreatedAt,
    UpdatedAt,
}
