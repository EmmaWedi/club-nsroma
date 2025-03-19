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
                    .table(CustomCocktails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CustomCocktails::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(CustomCocktails::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CustomCocktails::BranchId).uuid().not_null())
                    .col(
                        ColumnDef::new(CustomCocktails::CustomerId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CustomCocktails::Name).string().not_null())
                    .col(
                        ColumnDef::new(CustomCocktails::Ingredients)
                            .array(ColumnType::String(StringLen::default())),
                    )
                    .col(
                        ColumnDef::new(CustomCocktails::IsApproved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(CustomCocktails::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CustomCocktails::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CustomCocktails::Table, CustomCocktails::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CustomCocktails::Table, CustomCocktails::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CustomCocktails::Table, CustomCocktails::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomCocktails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CustomCocktails {
    Table,
    Id,
    OrganizationId,
    BranchId,
    CustomerId,
    Name,
    Ingredients,
    IsApproved,
    CreatedAt,
    UpdatedAt,
}
