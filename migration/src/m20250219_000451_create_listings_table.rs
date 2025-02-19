use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Listings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Listings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Listings::Title).string().not_null())
                    .col(ColumnDef::new(Listings::Description).text())
                    .col(ColumnDef::new(Listings::AuctionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-listings-auction_id")
                            .from(Listings::Table, Listings::AuctionId)
                            .to(Auctions::Table, Auctions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Listings::BasePrice).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Listings::AvailableVolume).integer().not_null().default(1))
                    .col(ColumnDef::new(Listings::DeletedAt).timestamp())
                    .col(ColumnDef::new(Listings::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(Listings::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Listings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Listings {
    Table,
    Id,
    Title,
    Description,
    AuctionId,
    StartTime,
    EndTime,
    BasePrice,
    AvailableVolume,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Auctions {
    Table,
    Id,
}