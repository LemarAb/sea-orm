use crate::{
    error::*, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QuerySelect,
    Select, SelectModel, SelectTwo, SelectTwoModel, Selector, SelectorTrait,
};
use sea_query::{Alias, SelectStatement};

/// A Trait for any type that can paginate results
pub trait AggregatorTrait<'db, C>
where
    C: ConnectionTrait,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C>;
}
/// Defined a structure to streamline aggregation queries
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
    pub async fn one<'a, C>(self, db: &C) -> Result<i64, DbErr>
    where
        C: ConnectionTrait,
    {
        let result = match db.query_one(self.query).await? {
            Some(res) => res,
            None => return Ok(0),
        };
        let aggregate =  result.try_get::<i64>("", "sub_query")?;
        Ok(aggregate)
    }


    /// Defined a structure to handle pagination of a result from a query operation on a Model
    pub fn count<T>(&mut self, col: T) -> &mut Aggregator<'db, C>
    where
        T: ColumnTrait,
    {
        let select = SelectStatement::new()
            .expr(col.count())
            .from_subquery(self.query.to_owned(), Alias::new("sub_query"))
            .to_owned();
        self.query = select;
        self
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
    M: FromQueryResult + Sized + Send + Sync + 'db,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C> {
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
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C> {
        Aggregator {
            query: self.query,
            db,
        }
    }
}
