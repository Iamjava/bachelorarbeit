use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;
use tokio::task;

struct Processor<T: std::clone::Clone> {
    receiver: Receiver<T>,
    sender: Sender<T>,
    closure_transform: fn(T) ->T,
    closure_predicate: fn(&T)->bool,
}

impl<T: std::marker::Sync + std::marker::Send +std::fmt::Debug + std::clone::Clone + 'static> Processor<T> {

    fn create(receiver: mpsc::Receiver<T>, closure_transform: fn (T) -> T,closure_predicate: fn(&T)->bool ) -> (Self, mpsc::Receiver<T>) {
        let (sender,outrecv) = mpsc::channel(100);
        (Self {
            receiver,
            sender:sender,
            closure_transform,
            closure_predicate,
        },outrecv)
    }

    async fn start(self,)->task::JoinHandle<String>{
        let mut receiver = self.receiver;
        let  sender = self.sender;

        let handle = task::spawn(async move {
            //let  mut agg = Vec::with_capacity(10);
            let mut i = 0;
            while i <= 3{
                let r = receiver.recv().await.unwrap();
                if (self.closure_predicate)(&r){
                    let processed = (self.closure_transform)(r);
                    sender.send(processed).await.unwrap();
                }
                i = i+1;
            }
            return "finished".to_string();
        });
        return handle;
    }

}



mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test1() {

        let (sender_1,  receiver_1) = mpsc::channel(20);

        let ( processor_1,  out1) = Processor::create(receiver_1,  |x| x +1, |_| true);
        let ( processor_2,mut out2) = Processor::create(out1,  |x| x - 10, |x|*x>0);
        let a = processor_1.start().await;
        let b = processor_2.start().await;

        task::spawn(async move {
            while let Some(v) = out2.recv().await {
                let processed = v;
                println!("Final recieved: {:?}",processed)
            }
        });

        sender_1.send(30 as u16).await.unwrap();
        sender_1.send(40 as u16).await.unwrap();
        sender_1.send(50 as u16).await.unwrap();
        sender_1.send(60 as u16).await.unwrap();
        sender_1.send(70 as u16).await.unwrap();
        a.await.unwrap();
        b.await.unwrap();
        //assert_eq!(receiver_2.recv().await.unwrap(),2);
    }

}