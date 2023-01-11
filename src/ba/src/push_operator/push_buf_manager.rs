use std::intrinsics::drop_in_place;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio::task;

struct Multiprocessor<T, U> {
    receiver_1:mpsc::Receiver<T>,
    sender: Arc<mpsc::Sender<T>>,
    closure: fn(Vec<T>,Vec<U>) -> T,
    handle: Option<JoinHandle<()>>,
}

impl<T:Clone + std::fmt::Debug + Send + Sync + 'static, U:Clone + std::fmt::Debug +Send + Sync + 'static> Multiprocessor<T, U> {
    fn create(receiver_1: mpsc::Receiver<T>, closure: fn (Vec<T>, Vec<U>) -> T ) -> (Self, mpsc::Receiver<T>) {
        let (sender,mut outrecv) = mpsc::channel(100);
        (Self {
            receiver_1,
            sender:Arc::new(sender),
            closure,
            handle: None,
        },outrecv)
    }

    async fn start(self,)->task::JoinHandle<String>{
        let sender = self.sender.clone();
        let closure = self.closure;
        let mut receiver = self.receiver_1;
        let mut sender = self.sender;

        let a  = task::spawn(async move {
            //let  mut agg = Vec::with_capacity(10);
            let mut i = 0;
            while i <= 4{
                let r = receiver.recv().await.unwrap();
                sender.send(r.clone()).await;
                i = i+1;
            }
            return "finished".to_string();

        });
        return a;


    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use tokio::sync::mpsc;
    use super::*;
    use tokio;
    use tokio::sync::watch;
    use std::thread;
    use tokio::task;

    #[tokio::test]
    async fn test1() {
        let (send, mut recv) = mpsc::channel(33);
        let handle = tokio::spawn(async move  {
            send.send(1u8).await.unwrap();
            send.send(2).await.unwrap();
            send.send(3).await.unwrap();
            drop(send);
        });
        // wait for the thread to join so we ensure the sender is dropped
        handle.await.unwrap();
        print!("{:?}",recv.recv().await);
    }

    #[tokio::test]
    async fn test3() {
         // Create the first channel
        let (tx1, mut rx1) = mpsc::channel(10);
        let tx2 = tx1.clone();
        // Send a message through the first channel
        let handle = task::spawn(async move {
            tx2.send("111").await.unwrap();
            tx2.send("AA").await.unwrap();
        });
        let handle3 = task::spawn(async move {
            tx1.send("222").await.unwrap();
            tx1.send("BBB").await.unwrap();
        });
        // Create the second channel
        let (tx2, mut rx2) = mpsc::channel(10);
        // Process the message in a separate task
        let handle = task::spawn(async move {
            while let Some(v) = rx1.recv().await {
                let processed = v.to_uppercase();
                tx2.send(processed).await.unwrap();
            }
        });
        let handle2 = task::spawn(async move {
            while let Some(v) = rx2.recv().await {
                println!("Processed value: {}", v);
            }
        });
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn test2() {
        let (sender_1, mut receiver_1) = mpsc::channel(20);
        let sender_x = sender_1.clone();

        let (mut processor,mut receiver_2) = Multiprocessor::create(receiver_1, |x, y|x.iter().sum::<i32>()+y.iter().sum::<i32>() );
        let h = processor.start().await;

        task::spawn(async move {
            while let Some(v) = receiver_2.recv().await {
                let processed = v;
                println!("Final recieved: {:?}",v)
            }
        });

        task::spawn(async move {
            sender_x.send(21).await;
        });

        sender_1.send(32).await;
        sender_1.send(32*2).await;
        sender_1.send(32*4).await;
        sender_1.send(32*8).await;
        let s = h.await;
        print!("{:?}",s.unwrap());



    }
}