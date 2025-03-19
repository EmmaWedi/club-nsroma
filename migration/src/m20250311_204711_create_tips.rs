use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_183402_create_orders::Orders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tips::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tips::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Tips::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Tips::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Tips::OrderId).uuid().not_null())
                    .col(
                        ColumnDef::new(Tips::Amount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tips::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tips::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tips::Table, Tips::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tips::Table, Tips::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tips::Table, Tips::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tips::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tips {
    Table,
    Id,
    OrganizationId,
    BranchId,
    OrderId,
    Amount,
    CreatedAt,
    UpdatedAt,
}
