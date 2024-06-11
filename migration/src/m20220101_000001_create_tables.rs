use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::{extension::postgres::Type, ColumnDef};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(
            Type::create()
                .as_enum(Role::Role)
                .values(vec![Role::Admin, Role::User])
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(User::Table)
                .col(
                    ColumnDef::new(User::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::Login).string_len(128).not_null())
                .col(ColumnDef::new(User::Password).string_len(255).not_null())
                .col(ColumnDef::new(User::Role).enumeration(Role::Role, vec![Role::Admin, Role::User]).not_null())
                .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;

        manager.get_connection().execute_unprepared(
            r#"CREATE UNIQUE INDEX idx_lower_login ON "user" (LOWER(login));"#
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            r#"DROP INDEX idx_lower_login;"#
        ).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(Role::Role).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Login,
    Password,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Role {
    Role,
    #[iden = "Admin"]
    Admin,
    #[iden = "User"]
    User,
}
