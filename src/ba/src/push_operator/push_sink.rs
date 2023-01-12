use crate::{ OperatorResult, PushOperator,};

pub struct PushSink {}

impl PushOperator for PushSink
{
    fn execute(&self, tuple: OperatorResult){
        match tuple {
            OperatorResult::SingleMatch(a)=>print!("{:?}",a),
            OperatorResult::MultiMatch(a)=>a.iter().for_each(|a|print!("{:?}",a))
        }
    }
}

