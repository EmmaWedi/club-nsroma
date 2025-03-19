use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_113056_create_departments::Departments,
    m20250312_114351_create_role_permissions::RolePermissions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Employees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Employees::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Employees::FirstName).string().not_null())
                    .col(ColumnDef::new(Employees::LastName).string().not_null())
                    .col(ColumnDef::new(Employees::Email).string().not_null())
                    .col(ColumnDef::new(Employees::Contact).string().not_null())
                    .col(
                        ColumnDef::new(Employees::EmployeeNumber)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Employees::Address).string().not_null())
                    .col(
                        ColumnDef::new(Employees::Gender).string().not_null().check(
                            Expr::col(Employees::Gender).is_in(vec![
                                GenderEnum::Male.as_str(),
                                GenderEnum::Female.as_str(),
                            ]),
                        ),
                    )
                    .col(
                        ColumnDef::new(Employees::DateOfBirth)
                            .date()
                            .not_null()
                            .check(
                                Expr::col(Employees::DateOfBirth).lte(Expr::cust("CURRENT_DATE")),
                            )
                            .check(
                                Expr::col(Employees::DateOfBirth).gte(Expr::value("1900-01-01")),
                            ),
                    )
                    .col(
                        ColumnDef::new(Employees::MaritalStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Employees::MaritalStatus).is_in(vec![
                                MaritalStatusEnum::Single.as_str(),
                                MaritalStatusEnum::Married.as_str(),
                                MaritalStatusEnum::Divorced.as_str(),
                                MaritalStatusEnum::Widowed.as_str(),
                            ]))
                            .default(MaritalStatusEnum::Single.as_str()),
                    )
                    .col(ColumnDef::new(Employees::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Employees::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Employees::DepartmentId).uuid().not_null())
                    .col(
                        ColumnDef::new(Employees::IdentificationType)
                            .string()
                            .check(Expr::col(Employees::IdentificationType).is_in(vec![
                                IdentificationTypeEnum::NationalId.as_str(),
                                IdentificationTypeEnum::Passport.as_str(),
                                IdentificationTypeEnum::DriverLicense.as_str(),
                            ])),
                    )
                    .col(ColumnDef::new(Employees::IdentificationNumber).string())
                    .col(ColumnDef::new(Employees::IdentificationImageUrl).string())
                    .col(ColumnDef::new(Employees::TaxIdentificationNumber).string())
                    .col(
                        ColumnDef::new(Employees::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Employees::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Employees::IsBookedOn)
                            .boolean()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Employees::EmployeeStartDate)
                            .date()
                            .not_null()
                            .check(
                                Expr::col(Employees::EmployeeStartDate)
                                    .lte(Expr::cust("CURRENT_DATE")),
                            )
                            .check(
                                Expr::col(Employees::EmployeeStartDate)
                                    .gte(Expr::value("1900-01-01")),
                            ),
                    )
                    .col(
                        ColumnDef::new(Employees::EmployeeEndDate)
                            .date()
                            .check(
                                Expr::col(Employees::EmployeeEndDate)
                                    .lte(Expr::cust("CURRENT_DATE")),
                            )
                            .check(
                                Expr::col(Employees::EmployeeEndDate)
                                    .gte(Expr::value("1900-01-01")),
                            ),
                    )
                    .col(
                        ColumnDef::new(Employees::EmployeeStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Employees::EmployeeStatus).is_in(vec![
                                EmployeeStatusEnum::Active.as_str(),
                                EmployeeStatusEnum::Dismissed.as_str(),
                                EmployeeStatusEnum::Resigned.as_str(),
                                EmployeeStatusEnum::Retired.as_str(),
                            ]))
                            .default(EmployeeStatusEnum::Active.as_str()),
                    )
                    .col(ColumnDef::new(Employees::RolePermissions).uuid().not_null())
                    .col(ColumnDef::new(Employees::Password).string())
                    .col(ColumnDef::new(Employees::Salt).string())
                    .col(ColumnDef::new(Employees::Session).string())
                    .col(
                        ColumnDef::new(Employees::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Employees::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Employees::DeletedAt)
                            .timestamp_with_time_zone()
                            .check(Expr::col(Employees::DeletedAt).lte(Expr::cust("CURRENT_DATE"))),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Employees::Table, Employees::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Employees::Table, Employees::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Employees::Table, Employees::DepartmentId)
                            .to(Departments::Table, Departments::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Employees::Table, Employees::RolePermissions)
                            .to(RolePermissions::Table, RolePermissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Employees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Employees {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Contact,
    EmployeeNumber,
    Address,
    Gender,
    DateOfBirth,
    MaritalStatus,
    OrganizationId,
    BranchId,
    DepartmentId,
    IdentificationType,
    IdentificationNumber,
    IdentificationImageUrl,
    TaxIdentificationNumber,
    IsDeleted,
    IsActive,
    IsBookedOn,
    EmployeeStartDate,
    EmployeeEndDate,
    EmployeeStatus,
    RolePermissions,
    Password,
    Salt,
    Session,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

enum GenderEnum {
    Male,
    Female,
}

impl GenderEnum {
    pub fn as_str(&self) -> &str {
        match self {
            GenderEnum::Male => "male",
            GenderEnum::Female => "female",
        }
    }
}

enum MaritalStatusEnum {
    Single,
    Married,
    Divorced,
    Widowed,
}

impl MaritalStatusEnum {
    pub fn as_str(&self) -> &str {
        match self {
            MaritalStatusEnum::Single => "single",
            MaritalStatusEnum::Married => "married",
            MaritalStatusEnum::Divorced => "divorced",
            MaritalStatusEnum::Widowed => "widowed",
        }
    }
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

enum EmployeeStatusEnum {
    Active,
    Dismissed,
    Resigned,
    Retired,
}

impl EmployeeStatusEnum {
    pub fn as_str(&self) -> &str {
        match self {
            EmployeeStatusEnum::Active => "active",
            EmployeeStatusEnum::Dismissed => "dismissed",
            EmployeeStatusEnum::Resigned => "resigned",
            EmployeeStatusEnum::Retired => "retired",
        }
    }
}
