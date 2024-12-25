use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut book_author = ForeignKey::create()
            .name("FK_book_author")
            .to(UserIden::Table, UserIden::UserId)
            .from(BookIden::Table, BookIden::Author)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut chapter_author = ForeignKey::create()
            .name("FK_chapter_author")
            .to(UserIden::Table, UserIden::UserId)
            .from(ChapterIden::Table, ChapterIden::Author)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut chapter_book = ForeignKey::create()
            .name("FK_chapter_book")
            .to(BookIden::Table, BookIden::BookId)
            .from(ChapterIden::Table, ChapterIden::Book)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(UserIden::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserIden::Email).string())
                    .col(ColumnDef::new(UserIden::IconUrl).string())
                    .col(ColumnDef::new(UserIden::Name).string())
                    .col(ColumnDef::new(UserIden::Pass).string())
                    .col(
                        ColumnDef::new(UserIden::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BookIden::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BookIden::Author).integer())
                    .col(
                        ColumnDef::new(BookIden::BookId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BookIden::Desc).string())
                    .col(ColumnDef::new(BookIden::IconUrl).string())
                    .col(ColumnDef::new(BookIden::Pubic).boolean())
                    .col(ColumnDef::new(BookIden::Published).date_time())
                    .col(ColumnDef::new(BookIden::Title).string())
                    .foreign_key(&mut book_author)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ChapterIden::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChapterIden::Author).integer())
                    .col(ColumnDef::new(ChapterIden::Book).integer())
                    .col(ColumnDef::new(ChapterIden::Pubic).boolean())
                    .col(ColumnDef::new(ChapterIden::Published).date_time())
                    .col(ColumnDef::new(ChapterIden::Text).string())
                    .col(ColumnDef::new(ChapterIden::Title).string())
                    .foreign_key(&mut chapter_author)
                    .foreign_key(&mut chapter_book)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden, Debug)]
pub enum UserIden {
    Name,
    Pass,
    Email,
    UserId,
    IconUrl,
    Table,
}

#[derive(Iden, Debug)]
pub enum BookIden {
    Title,
    Desc,
    Published,
    Pubic,
    BookId,
    Author,
    IconUrl,
    Table,
}

#[derive(Iden, Debug)]
pub enum ChapterIden {
    Title,
    Text,
    Published,
    Pubic,
    Author,
    Book,
    Table,
}
