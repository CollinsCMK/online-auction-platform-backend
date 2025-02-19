pub use sea_orm_migration::prelude::*;

// mod m20220101_000001_create_table;
mod m20250219_000030_create_auctions_table;
mod m20250219_000451_create_listings_table;
mod m20250219_001043_create_users_table;
mod m20250219_001443_create_bids_table;
mod m20250219_002133_create_auction_results_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250219_000030_create_auctions_table::Migration),
            Box::new(m20250219_000451_create_listings_table::Migration),
            Box::new(m20250219_001043_create_users_table::Migration),
            Box::new(m20250219_001443_create_bids_table::Migration),
            Box::new(m20250219_002133_create_auction_results_table::Migration),
        ]
    }
}
