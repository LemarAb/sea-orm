use crate::{error::*, ColumnTrait, ConnectionTrait};
use sea_query::SelectStatement;
#[async_trait::async_trait]
pub trait AggregatorTrait {
    fn aggregate(self, db: &'db C) -> Aggregator<'db, C>;
}

pub struct Aggregator<'db, C>
where
    C: ConnectionTrait,
{
    pub(crate) query: SelectStatement,
    pub(crate) db: &'db C,
}

impl<'db, C> Aggregator<'db, C> {
    pub fn count<T>(col: T) -> Result<i64, DbErr>
    where
        T: ColumnTrait,
    {
    }
}
