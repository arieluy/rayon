use super::ParallelIterator;
use super::len::*;
use super::internal::*;
use super::util::*;

pub fn for_each<PAR_ITER,OP,T>(pi: PAR_ITER, op: &OP)
    where PAR_ITER: ParallelIterator<Item=T>,
          OP: Fn(T) + Sync,
          T: Send,
{
    let consumer = ForEachConsumer { op: op };
    pi.drive_unindexed(consumer)
}

struct ForEachConsumer<'f, OP: 'f> {
    op: &'f OP,
}

impl<'f, OP, ITEM> Consumer<ITEM> for ForEachConsumer<'f, OP>
    where OP: Fn(ITEM) + Sync,
{
    type Folder = ForEachConsumer<'f, OP>;
    type Reducer = NoopReducer;
    type Result = ();

    fn cost(&mut self, cost: f64) -> f64 {
        // This isn't quite right, as we will do more than O(n) reductions, but whatever.
        cost * FUNC_ADJUSTMENT
    }

    fn split_at(self, _index: usize) -> (Self, Self, NoopReducer) {
        (self.split(), self.split(), NoopReducer)
    }

    fn fold(self) -> Self {
        self
    }
}

impl<'f, OP, ITEM> Folder<ITEM> for ForEachConsumer<'f, OP>
    where OP: Fn(ITEM) + Sync,
{
    type Result = ();

    fn consume(self, item: ITEM) -> Self {
        (self.op)(item);
        self
    }

    fn complete(self) {
    }
}

impl<'f, OP, ITEM> UnindexedConsumer<ITEM> for ForEachConsumer<'f, OP>
    where OP: Fn(ITEM) + Sync,
{
    fn split(&self) -> Self {
        ForEachConsumer { op: self.op }
    }

    fn reducer(&self) -> NoopReducer {
        NoopReducer
    }
}
