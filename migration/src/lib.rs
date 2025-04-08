pub use sea_orm_migration::prelude::*;

mod m20250310_181001_create_media;
mod m20250311_100140_create_countries;
mod m20250311_102524_create_organizations;
mod m20250311_110557_create_contact_lists;
mod m20250311_111857_create_branches;
mod m20250311_113056_create_departments;
mod m20250311_114321_create_staff;
mod m20250311_123539_create_schedule;
mod m20250311_125624_create_events;
mod m20250311_131422_create_freebies;
mod m20250311_132020_create_stock_drinks;
mod m20250311_133559_create_stock_foods;
mod m20250311_134128_create_category;
mod m20250311_134626_create_suppliers;
mod m20250311_135726_create_customers;
mod m20250311_140939_create_custom_cocktails;
mod m20250311_142305_create_wallets;
mod m20250311_165323_create_transactions;
mod m20250311_171654_create_accounts;
mod m20250311_172648_create_wallet_topups;
mod m20250311_173900_create_externals;
mod m20250311_174956_create_bookings;
mod m20250311_180431_create_guest_bookings;
mod m20250311_183402_create_orders;
mod m20250311_203009_create_order_items;
mod m20250311_204711_create_tips;
mod m20250311_210343_create_carousels;
mod m20250312_092226_create_ticket_events;
mod m20250312_094550_create_tickets;
mod m20250312_100734_create_staff_shifts;
mod m20250312_101358_create_staff_leaves;
mod m20250312_105209_create_till_sessions;
mod m20250312_111549_create_promotions;
mod m20250312_112345_create_referrals;
mod m20250312_113411_create_roles;
mod m20250312_113839_create_permissions;
mod m20250312_114351_create_role_permissions;
mod m20250317_161334_create_users;
mod m20250408_170934_create_ban_records;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250310_181001_create_media::Migration),
            Box::new(m20250311_100140_create_countries::Migration),
            Box::new(m20250311_102524_create_organizations::Migration),
            Box::new(m20250312_113411_create_roles::Migration),
            Box::new(m20250312_113839_create_permissions::Migration),
            Box::new(m20250312_114351_create_role_permissions::Migration),
            Box::new(m20250311_110557_create_contact_lists::Migration),
            Box::new(m20250311_111857_create_branches::Migration),
            Box::new(m20250311_113056_create_departments::Migration),
            Box::new(m20250311_114321_create_staff::Migration),
            Box::new(m20250311_123539_create_schedule::Migration),
            Box::new(m20250311_125624_create_events::Migration),
            Box::new(m20250311_134128_create_category::Migration),
            Box::new(m20250312_105209_create_till_sessions::Migration),
            Box::new(m20250311_131422_create_freebies::Migration),
            Box::new(m20250311_134626_create_suppliers::Migration),
            Box::new(m20250311_132020_create_stock_drinks::Migration),
            Box::new(m20250311_133559_create_stock_foods::Migration),
            Box::new(m20250311_135726_create_customers::Migration),
            Box::new(m20250311_140939_create_custom_cocktails::Migration),
            Box::new(m20250311_142305_create_wallets::Migration),
            Box::new(m20250311_165323_create_transactions::Migration),
            Box::new(m20250311_171654_create_accounts::Migration),
            Box::new(m20250311_172648_create_wallet_topups::Migration),
            Box::new(m20250311_173900_create_externals::Migration),
            Box::new(m20250311_174956_create_bookings::Migration),
            Box::new(m20250311_180431_create_guest_bookings::Migration),
            Box::new(m20250311_183402_create_orders::Migration),
            Box::new(m20250311_203009_create_order_items::Migration),
            Box::new(m20250311_204711_create_tips::Migration),
            Box::new(m20250311_210343_create_carousels::Migration),
            Box::new(m20250312_092226_create_ticket_events::Migration),
            Box::new(m20250312_094550_create_tickets::Migration),
            Box::new(m20250312_100734_create_staff_shifts::Migration),
            Box::new(m20250312_101358_create_staff_leaves::Migration),
            Box::new(m20250312_111549_create_promotions::Migration),
            Box::new(m20250312_112345_create_referrals::Migration),
            Box::new(m20250317_161334_create_users::Migration),
            Box::new(m20250408_170934_create_ban_records::Migration),
        ]
    }
}
