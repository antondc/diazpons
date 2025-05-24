use super::super::{PostgreSQLClient, ToSqlResult, ToSqlType, ToSqlValue};
use crate::domain::board::{Board, IBoardRepository};
use crate::domain::user::Role;
use crate::types::{Errors, Meta, Result};
use async_trait::async_trait;

#[derive(Copy, Clone)]
pub struct PostgreSQLBoardRepository;

#[async_trait]
impl IBookRepository for JsonBookRepository {
    async fn book_get_all(&self, slug: &str, roles: Vec<Role>) -> Result<(Meta, Vec<Book>)> {
        // let client = PostgreSQLClient::new().await?;

        // let result = client
        //   .query("select * from board_get_all($1, $2)", &[&slug, &roles])
        //   .await
        //   .map_err(|error| Errors::new(Errors::Database, Some(error.to_string())))?;

        // let items: Vec<Board> = result.into_iter().map(Board::from).collect();
        // let count = items.first().map(|item| item.count.unwrap());

        // let meta = Meta {
        //   total_items: count,
        //   offset: None,
        //   size: None,
        //   sort: None,
        // };

        // Ok((meta, items))
        Ok(())
    }
}
