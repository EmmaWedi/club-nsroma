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
                    .table(GuestBookings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GuestBookings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(GuestBookings::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GuestBookings::BranchId).uuid().not_null())
                    .col(ColumnDef::new(GuestBookings::BookedBy).uuid().not_null())
                    .col(ColumnDef::new(GuestBookings::BookingId).uuid().not_null())
                    .col(ColumnDef::new(GuestBookings::AttendeeId).uuid().not_null())
                    .col(
                        ColumnDef::new(GuestBookings::BookingReference)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GuestBookings::QrCodeId).string())
                    .col(
                        ColumnDef::new(GuestBookings::IsBookOn)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(GuestBookings::IsCancelled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(GuestBookings::BookedOnTime).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(GuestBookings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GuestBookings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuestBookings::Table, GuestBookings::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuestBookings::Table, GuestBookings::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuestBookings::Table, GuestBookings::BookedBy)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GuestBookings::Table, GuestBookings::AttendeeId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GuestBookings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GuestBookings {
    Table,
    Id,
    OrganizationId,
    BranchId,
    BookedBy,
    BookingId,
    AttendeeId,
    BookingReference,
    QrCodeId,
    IsBookOn,
    IsCancelled,
    BookedOnTime,
    CreatedAt,
    UpdatedAt,
}
