use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuctionResults::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuctionResults::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuctionResults::ListingId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-auction_results-listing_id")
                            .from(AuctionResults::Table, AuctionResults::ListingId)
                            .to(Listings::Table, Listings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(AuctionResults::WinningBidId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-auction_results-winning_bid_id")
                            .from(AuctionResults::Table, AuctionResults::WinningBidId)
                            .to(Bids::Table, Bids::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(AuctionResults::WinningUserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-auction_results-winning_user_id")
                            .from(AuctionResults::Table, AuctionResults::WinningUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(AuctionResults::DeletedAt).timestamp())
                    .col(ColumnDef::new(AuctionResults::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(AuctionResults::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuctionResults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AuctionResults {
    Table,
    Id,
    ListingId,
    WinningBidId,
    WinningUserId,
    Amount,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Listings {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Bids {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}