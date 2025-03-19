use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Permissions::Action)
                            .array(ColumnType::String(StringLen::default())),
                    )
                    .col(
                        ColumnDef::new(Permissions::Model)
                            .array(ColumnType::String(StringLen::default())),
                    )
                    .col(
                        ColumnDef::new(Permissions::ModelAction)
                            .array(ColumnType::String(StringLen::default())),
                    )
                    .col(
                        ColumnDef::new(Permissions::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Permissions::DeletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Permissions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Permissions::UpdatedAt)
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
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    Id,
    Action,      //(read, create, update, delete)
    Model,       //corresponds with schema names
    ModelAction, //(eg Customers::Read, Employees::Update)
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
