use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Customers::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Customers::FirstName).string().not_null())
                    .col(ColumnDef::new(Customers::LastName).string().not_null())
                    .col(ColumnDef::new(Customers::Contact).string().not_null())
                    .col(ColumnDef::new(Customers::Email).string())
                    .col(
                        ColumnDef::new(Customers::IsStudent)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Customers::StudentIdNumber).string())
                    .col(ColumnDef::new(Customers::StudentIdImageUrl).string())
                    .col(
                        ColumnDef::new(Customers::CustomerNumber)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Customers::IdentificationType)
                            .string()
                            .check(Expr::col(Customers::IdentificationType).is_in(vec![
                                IdentificationTypeEnum::NationalId.as_str(),
                                IdentificationTypeEnum::Passport.as_str(),
                                IdentificationTypeEnum::DriverLicense.as_str(),
                            ])),
                    )
                    .col(ColumnDef::new(Customers::IdentificationNumber).string())
                    .col(ColumnDef::new(Customers::IdentificationImageUrl).string())
                    .col(
                        ColumnDef::new(Customers::IsBlocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Customers::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Customers::BlockedReason).string())
                    .col(
                        ColumnDef::new(Customers::IsBanned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Customers::BannedReason).string())
                    .col(ColumnDef::new(Customers::BannedDuration).integer())
                    .col(
                        ColumnDef::new(Customers::BannedAt)
                            .timestamp_with_time_zone()
                            .check(Expr::col(Customers::BannedAt).lte(Expr::cust("CURRENT_DATE"))),
                    )
                    .col(ColumnDef::new(Customers::BannedUntil).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Customers::DateOfBirth)
                        .date()
                        .check(
                            Expr::col(Customers::DateOfBirth).lte(Expr::cust("CURRENT_DATE")),
                        ),
                    )
                    .col(
                        ColumnDef::new(Customers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Customers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Customers::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Customers {
    Table,
    Id,
    FirstName,
    LastName,
    Contact,
    Email,
    IsStudent,
    StudentIdNumber,
    StudentIdImageUrl,
    CustomerNumber,
    IdentificationType,
    IdentificationNumber,
    IdentificationImageUrl,
    IsBlocked,
    IsDeleted,
    BlockedReason,
    IsBanned,
    BannedReason,
    BannedDuration,
    BannedAt,
    BannedUntil,
    DateOfBirth,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

enum IdentificationTypeEnum {
    NationalId,
    Passport,
    DriverLicense,
}

impl IdentificationTypeEnum {
    pub fn as_str(&self) -> &str {
        match self {
            IdentificationTypeEnum::NationalId => "national_id",
            IdentificationTypeEnum::Passport => "passport",
            IdentificationTypeEnum::DriverLicense => "driver_license",
        }
    }
}
