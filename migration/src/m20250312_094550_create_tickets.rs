use sea_orm_migration::prelude::*;

use crate::m20250312_092226_create_ticket_events::TicketEvents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tickets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tickets::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Tickets::ImageUrl).string())
                    .col(
                        ColumnDef::new(Tickets::Price)
                            .decimal()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Tickets::IsDiscountApplied)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Tickets::DiscountRate)
                            .decimal()
                            .not_null()
                            .default(0.0),
                    )
                    .col(ColumnDef::new(Tickets::Slug).string())
                    .col(ColumnDef::new(Tickets::Sku).string())
                    .col(
                        ColumnDef::new(Tickets::IsApproved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Tickets::ApprovedBy).uuid())
                    .col(ColumnDef::new(Tickets::IsActive).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(Tickets::EndDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tickets::Location).string())
                    .col(ColumnDef::new(Tickets::TicketType).string())
                    .col(ColumnDef::new(Tickets::TicketEventId).uuid().not_null())
                    .col(
                        ColumnDef::new(Tickets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tickets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tickets::Table, Tickets::TicketEventId)
                            .to(TicketEvents::Table, TicketEvents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tickets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tickets {
    Table,
    Id,
    ImageUrl,
    Price,
    IsDiscountApplied,
    DiscountRate,
    Slug,
    Sku,
    IsApproved,
    ApprovedBy,
    IsActive,
    EndDate,
    Location,
    TicketType,
    TicketEventId,
    CreatedAt,
    UpdatedAt,
}
