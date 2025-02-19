//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bids")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub listing_id: i32,
    pub user_id: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub deleted_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auction_results::Entity")]
    AuctionResults,
    #[sea_orm(
        belongs_to = "super::listings::Entity",
        from = "Column::ListingId",
        to = "super::listings::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Listings,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::auction_results::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuctionResults.def()
    }
}

impl Related<super::listings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listings.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
