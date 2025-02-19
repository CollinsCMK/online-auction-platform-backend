use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Auctions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Auctions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Auctions::Title).string().not_null())
                    .col(ColumnDef::new(Auctions::StartTime).timestamp())
                    .col(ColumnDef::new(Auctions::EndTime).timestamp())
                    .col(ColumnDef::new(Auctions::DeletedAt).timestamp())
                    .col(ColumnDef::new(Auctions::CreatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(Auctions::UpdatedAt).timestamp().default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Auctions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Auctions {
    Table,
    Id,
    Title,
    StartTime,
    EndTime,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
