use sea_orm_migration::prelude::*;

use crate::m20250311_102524_create_organizations::Organizations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactLists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactLists::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(ContactLists::Address).string().not_null())
                    .col(ColumnDef::new(ContactLists::ContactType).string().not_null())
                    .col(
                        ColumnDef::new(ContactLists::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ContactLists::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(ContactLists::Description).string())
                    .col(ColumnDef::new(ContactLists::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(ContactLists::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ContactLists::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ContactLists::Table, ContactLists::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContactLists::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ContactLists {
    Table,
    Id,
    Address,
    ContactType,
    IsDeleted,
    IsActive,
    Description,
    OrganizationId,
    CreatedAt,
    UpdatedAt
}
