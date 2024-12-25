use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut book_author = ForeignKey::create()
            .name("FK_book_author")
            .to(User::Table, User::Id)
            .from(Book::Table, Book::Author)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut chapter_author = ForeignKey::create()
            .name("FK_chapter_author")
            .to(User::Table, User::Id)
            .from(Chapter::Table, Chapter::Author)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let mut chapter_book = ForeignKey::create()
            .name("FK_chapter_book")
            .to(Book::Table, Book::Id)
            .from(Chapter::Table, Chapter::Book)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::IconUrl).string())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Pass).string().not_null())
                    .col(
                        ColumnDef::new(User::Id)
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
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Author).integer().not_null())
                    .col(
                        ColumnDef::new(Book::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Book::Desc).string())
                    .col(ColumnDef::new(Book::IconUrl).string())
                    .col(ColumnDef::new(Book::Pubic).boolean().not_null())
                    .col(ColumnDef::new(Book::Published).date_time())
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .foreign_key(&mut book_author)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chapter::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Chapter::Author).integer().not_null())
                    .col(ColumnDef::new(Chapter::Book).integer())
                    .col(ColumnDef::new(Chapter::Pubic).boolean())
                    .col(ColumnDef::new(Chapter::Published).date_time())
                    .col(ColumnDef::new(Chapter::Text).string().not_null())
                    .col(ColumnDef::new(Chapter::Title).string().not_null())
                    .col(
                        ColumnDef::new(Chapter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .foreign_key(&mut chapter_author)
                    .foreign_key(&mut chapter_book)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden, Debug)]
pub enum User {
    Name,
    Pass,
    Email,
    Id,
    IconUrl,
    Table,
}

#[derive(Iden, Debug)]
pub enum Book {
    Title,
    Desc,
    Published,
    Pubic,
    Id,
    Author,
    IconUrl,
    Table,
}

#[derive(Iden, Debug)]
pub enum Chapter {
    Title,
    Text,
    Published,
    Pubic,
    Author,
    Book,
    Id,
    Table,
}
