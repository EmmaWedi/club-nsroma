use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TicketEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicketEvents::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(TicketEvents::Title).string().not_null())
                    .col(
                        ColumnDef::new(TicketEvents::Description)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketEvents::Location).string())
                    .col(
                        ColumnDef::new(TicketEvents::EventDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TicketEvents::TicketPrice)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketEvents::Capacity).integer().default(0))
                    .col(
                        ColumnDef::new(TicketEvents::EventStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(TicketEvents::EventStatus).is_in(vec![
                                EventStatusEnum::Active.as_str(),
                                EventStatusEnum::Elapsed.as_str(),
                                EventStatusEnum::Full.as_str(),
                            ]))
                            .default(EventStatusEnum::Active.as_str()),
                    )
                    .col(
                        ColumnDef::new(TicketEvents::IsCancelled)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(TicketEvents::CancelledDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(TicketEvents::MinAgeLimit)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TicketEvents::MaxAgeLimit)
                            .integer()
                            .default(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TicketEvents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TicketEvents::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TicketEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TicketEvents {
    Table,
    Id,
    Title,
    Description,
    Location,
    EventDate,
    TicketPrice,
    Capacity,
    EventStatus,
    IsCancelled,
    CancelledDate,
    MinAgeLimit,
    MaxAgeLimit,
    CreatedAt,
    UpdatedAt,
}

enum EventStatusEnum {
    Active,
    Elapsed,
    Full,
}

impl EventStatusEnum {
    fn as_str(&self) -> &'static str {
        match self {
            EventStatusEnum::Active => "active",
            EventStatusEnum::Elapsed => "elapsed",
            EventStatusEnum::Full => "full",
        }
    }
}
