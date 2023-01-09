use crate::{Chunk, DynTuple, Operator, TupleChunk, DynValue, OperatorResult, PushOperator};
use crate::{ CHUNK_SIZE,};
use crate::push_operator::push_sink::PushSink;

type Predicate = fn(&DynTuple)->bool;

pub struct PushFilter<O: PushOperator> {
    sink: O,
    predicate: Predicate,
}

impl<O: PushOperator> PushFilter<O>{
    fn new(o: O, predicate: Predicate)->Self{
        Self{
            sink: o,
            predicate,
        }
    }
}

impl<O: PushOperator> PushOperator for PushFilter<O>{
    fn execute(&self, tuple: OperatorResult){
        match tuple {
            OperatorResult::SingleMatch(ref a)=>{
                if (self.predicate)(a) {
                    self.sink.execute(tuple);
                }
            },
            OperatorResult::MultiMatch(a)=>a.iter().for_each(|a|print!("IMPLEMENT ME")),
        }
    }
}

impl Default for PushFilter<PushSink>
{
    fn default() -> Self {
        Self{
            sink: PushSink {},
            predicate: |_a|true,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;
    use crate::operator::filter::*;
    use crate::operator::scan::Scan;

    #[test]
    fn test_filter_1() {
        let fil = PushFilter::default();
        let fil2 = PushFilter::new(fil, |a| true);
        let opres = OperatorResult::SingleMatch(vec![DynValue::TBool(true),DynValue::TFloat(1.2)]);
        fil2.execute(opres);
    }
}
