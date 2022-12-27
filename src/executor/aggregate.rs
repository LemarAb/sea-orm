use crate::{
    error::*, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, Select, SelectModel,
    SelectTwo, SelectTwoModel, Selector, SelectorTrait,
};
use sea_query::SelectStatement;

#[async_trait::async_trait]
pub trait AggregatorTrait<'db, C>
where
    C: ConnectionTrait,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C>;
}

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
    pub fn count<T>(col: T) -> Result<i64, DbErr>
    where
        T: ColumnTrait,
    {
    }
}

impl<'db, C, S> AggregatorTrait<'db, C> for Selector<S>
where
    C: ConnectionTrait,
    S: SelectorTrait + Send + Sync + 'db,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C> {
        todo!()
    }
}

impl<'db, C, E, M> AggregatorTrait<'db, C> for Select<E>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
{
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C> {
        todo!()
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
        todo!()
    }
}
