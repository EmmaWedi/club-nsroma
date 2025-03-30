use sea_orm_migration::prelude::*;

use crate::{
    m20250311_102524_create_organizations::Organizations,
    m20250311_111857_create_branches::Branches, m20250311_135726_create_customers::Customers,
    m20250311_165323_create_transactions::Transactions,
    m20250312_105209_create_till_sessions::TillSessions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Orders::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Orders::CustomerId).uuid().not_null())
                    .col(ColumnDef::new(Orders::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Orders::BranchId).uuid().not_null())
                    .col(ColumnDef::new(Orders::OrderNumber).string().not_null())
                    .col(ColumnDef::new(Orders::PaymentId).uuid().not_null())
                    .col(
                        ColumnDef::new(Orders::TotalAmount)
                            .decimal()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::IsDiscountApplied)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::IsCancelled)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::ServedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(Orders::OrderStatus)
                            .string()
                            .not_null()
                            .check(Expr::col(Orders::OrderStatus).is_in(vec![
                                OrderStatusEnum::Pending.as_str(),
                                OrderStatusEnum::Processing.as_str(),
                                OrderStatusEnum::Completed.as_str(),
                                OrderStatusEnum::Cancelled.as_str(),
                                OrderStatusEnum::Rejected.as_str(),
                            ]))
                            .default(OrderStatusEnum::Pending.as_str()),
                    )
                    .col(ColumnDef::new(Orders::TillSessionId).uuid())
                    .col(
                        ColumnDef::new(Orders::IsTipApplied)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::TipAmount).decimal().default(0.0))
                    .col(
                        ColumnDef::new(Orders::OrderState)
                            .string()
                            .not_null()
                            .check(Expr::col(Orders::OrderState).is_in(vec![
                                OrderStateEnum::PreOrdered.as_str(),
                                OrderStateEnum::InstantOrder.as_str(),
                            ]))
                            .default(OrderStateEnum::InstantOrder.as_str()),
                    )
                    .col(
                        ColumnDef::new(Orders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Orders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::BranchId)
                            .to(Branches::Table, Branches::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::TillSessionId)
                            .to(TillSessions::Table, TillSessions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::PaymentId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Orders {
    Table,
    Id,
    CustomerId,
    OrganizationId,
    BranchId,
    OrderNumber,
    TotalAmount,
    PaymentId,
    IsDiscountApplied,
    IsCancelled,
    ServedBy,
    OrderStatus,
    TillSessionId,
    IsTipApplied,
    TipAmount,
    OrderState,
    CreatedAt,
    UpdatedAt,
}

enum OrderStatusEnum {
    Pending,
    Processing,
    Completed,
    Rejected,
    Cancelled,
}

impl OrderStatusEnum {
    fn as_str(&self) -> &'static str {
        match self {
            OrderStatusEnum::Pending => "PENDING",
            OrderStatusEnum::Processing => "PROCESSING",
            OrderStatusEnum::Completed => "COMPLETED",
            OrderStatusEnum::Rejected => "REJECTED",
            OrderStatusEnum::Cancelled => "CANCELLED",
        }
    }
}

enum OrderStateEnum {
    PreOrdered,
    InstantOrder,
}

impl OrderStateEnum {
    fn as_str(&self) -> &'static str {
        match self {
            OrderStateEnum::InstantOrder => "INSTANTORDER",
            OrderStateEnum::PreOrdered => "PREORDERED",
        }
    }
}
