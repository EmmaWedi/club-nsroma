use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_125624_create_events::Events,
    m20250311_135726_create_customers::Customers,
    m20250311_165323_create_transactions::Transactions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bookings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Bookings::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::CustomerId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::EventId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::PaymentId).uuid().not_null())
                    .col(
                        ColumnDef::new(Bookings::IsCancelled)
                            .boolean()
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .col(ColumnDef::new(Bookings::BookingNumber).string().not_null())
                    .col(
                        ColumnDef::new(Bookings::BookingReference)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bookings::BookingParty)
                            .string()
                            .not_null()
                            .check(Expr::col(Bookings::BookingParty).is_in(vec![
                                BookingPartyEnum::Individual.as_str(),
                                BookingPartyEnum::Group.as_str(),
                            ]))
                            .default(BookingPartyEnum::Individual.as_str()),
                    )
                    .col(ColumnDef::new(Bookings::IsBookOn).timestamp_with_time_zone())
                    .col(ColumnDef::new(Bookings::QrCodeId).string())
                    .col(
                        ColumnDef::new(Bookings::UnitAmount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bookings::TotalAmount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Bookings::BookedOnTime).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Bookings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Bookings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bookings::Table, Bookings::PaymentId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Bookings {
    Table,
    Id,
    OrganizationId,
    BranchId,
    CustomerId,
    EventId,
    PaymentId,
    IsCancelled,
    BookingNumber,
    BookingReference,
    BookingParty,
    IsBookOn,
    QrCodeId,
    UnitAmount,
    TotalAmount,
    BookedOnTime,
    CreatedAt,
    UpdatedAt,
}

enum BookingPartyEnum {
    Individual,
    Group,
}

impl BookingPartyEnum {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Individual => "INDIVIDUAL",
            Self::Group => "GROUP",
        }
    }
}
