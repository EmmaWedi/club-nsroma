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
                    .col(ColumnDef::new(Customers::FirstName).string())
                    .col(ColumnDef::new(Customers::LastName).string())
                    .col(ColumnDef::new(Customers::Contact).string().not_null())
                    .col(ColumnDef::new(Customers::Email).string())
                    .col(
                        ColumnDef::new(Customers::IsStudent)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Customers::StudentIdNumber).string())
                    .col(ColumnDef::new(Customers::StudentIdImageId).string())
                    .col(
                        ColumnDef::new(Customers::IsStudentIdVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
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
                    .col(ColumnDef::new(Customers::IdentificationImageId).string())
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
                    .col(ColumnDef::new(Customers::Password).string())
                    .col(ColumnDef::new(Customers::Salt).string())
                    .col(ColumnDef::new(Customers::Session).string())
                    .col(ColumnDef::new(Customers::StudentEndYear).integer())
                    .col(ColumnDef::new(Customers::Username).string())
                    .col(ColumnDef::new(Customers::IsActive).boolean().default(false).not_null())
                    .col(
                        ColumnDef::new(Customers::DateOfBirth)
                        .date()
                        .check(
                            Expr::col(Customers::DateOfBirth).lte(Expr::cust("CURRENT_DATE")),
                        ),
                    )
                    .col(
                        ColumnDef::new(Customers::IsIdVerified)
                            .boolean()
                            .not_null()
                            .default(false),
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
    StudentIdImageId,
    IsStudentIdVerified,
    CustomerNumber,
    IdentificationType,
    IdentificationNumber,
    IdentificationImageId,
    IsIdVerified,
    IsBlocked,
    IsDeleted,
    IsActive,
    BlockedReason,
    IsBanned,
    DateOfBirth,
    Password,
    Salt,
    Session,
    StudentEndYear,
    Username,
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
            IdentificationTypeEnum::NationalId => "NATIONALID",
            IdentificationTypeEnum::Passport => "PASSPORT",
            IdentificationTypeEnum::DriverLicense => "DRIVERLICENSE",
        }
    }
}