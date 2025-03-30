use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Countries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Countries::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Countries::Name).string().not_null())
                    .col(ColumnDef::new(Countries::FlagId).string())
                    .col(ColumnDef::new(Countries::CallCode).string())
                    .col(ColumnDef::new(Countries::CurrencyCode).string())
                    .col(ColumnDef::new(Countries::Currency).string())
                    .col(ColumnDef::new(Countries::IsoCode).string())
                    .col(
                        ColumnDef::new(Countries::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Countries::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Countries::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Countries::UpdatedAt)
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
            .drop_table(Table::drop().table(Countries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Countries {
    Table,
    Id,
    Name,
    FlagId,
    CallCode,
    CurrencyCode,
    Currency,
    IsoCode,
    IsActive,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
