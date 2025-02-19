use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bids::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bids::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bids::ListingId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bids-listing_id")
                            .from(Bids::Table, Bids::ListingId)
                            .to(Listings::Table, Listings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Bids::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-bids-user_id")
                            .from(Bids::Table, Bids::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .col(ColumnDef::new(Bids::Amount).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Bids::DeletedAt).timestamp())
                    .col(ColumnDef::new(Bids::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(Bids::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bids::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Bids {
    Table,
    Id,
    ListingId,
    UserId,
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
enum Users {
    Table,
    Id,
}