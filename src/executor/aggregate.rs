use crate::{
    error::*, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QuerySelect,
    Select, SelectModel, SelectTwo, SelectTwoModel, Selector, SelectorTrait,
};
use sea_query::SelectStatement;

#[async_trait::async_trait]
pub trait AggregatorTrait<'db, C>
where
    C: ConnectionTrait,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C>;
}

#[derive(Debug)]
pub struct Aggregator<'db, C>
where
    C: ConnectionTrait,
{
    pub(crate) query: SelectStatement,
    pub(crate) db: &'db C,
}

impl<'db, C> Aggregator<'db, C>
where
    C: ConnectionTrait,
{
    pub async fn count<T>(&self, col: T) -> Result<i64, DbErr>
    where
        T: ColumnTrait,
    {
        let builder = self.db.get_database_backend();
        let stmt = builder.build(
            SelectStatement::new()
                .expr(col.count())
                .from(col.entity_name()),
        );

        let result = match self.db.query_one(stmt).await? {
            Some(res) => res,
            None => return Ok(0),
        };
        let count = match builder {
            DbBackend::Postgres => {
                result.try_get::<i64>("", &format!("COUNT({})", col.as_str()))?
            }
            _ => result.try_get::<i64>("", &format!("COUNT({})", col.as_str()))?,
        };
        Ok(2)
    }
}

impl<'db, C, S> AggregatorTrait<'db, C> for Selector<S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C> {
        Aggregator {
            query: self.query,
            db,
        }
    }
}

impl<'db, C, E, M> AggregatorTrait<'db, C> for Select<E>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
{
    fn aggregate(self, db: &'db C) -> Aggregator<C> {
        Aggregator {
            query: self.query,
            db,
        }
    }
}

impl<'db, C, M, N, E, F> AggregatorTrait<'db, C> for SelectTwo<E, F>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    F: EntityTrait<Model = N>,
    M: FromQueryResult + Sized + Send + Sync + 'db,
    N: FromQueryResult + Sized + Send + Sync + 'db,
{
    fn aggregate(self, db: &'db C) -> Aggregator<C> {
        Aggregator {
            query: self.query,
            db,
        }
    }
}
