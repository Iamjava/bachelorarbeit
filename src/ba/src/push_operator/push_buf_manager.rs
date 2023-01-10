use std::intrinsics::drop_in_place;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;

struct Multiprocessor<T, U> {
    receiver_1: watch::Receiver<T>,
    receiver_2: watch::Receiver<U>,
    sender: Arc<watch::Sender<T>>,
    closure: fn(Vec<T>,Vec<U>) -> T,
    handle: Option<JoinHandle<()>>,
}

impl<T:Clone + std::fmt::Debug + Send + Sync + 'static, U:Clone + std::fmt::Debug +Send + Sync + 'static> Multiprocessor<T, U> {
    fn new(receiver_1:watch::Receiver<T>, receiver_2: watch::Receiver<U>, sender: watch::Sender<T>, closure: fn (Vec<T>,Vec<U>) -> T ) -> Self {
        Self {
            receiver_1,
            receiver_2,
            sender:Arc::new(sender),
            closure,
            handle: None,
        }
    }

    async fn start(&mut self,) {
        let sender = self.sender.clone();
        let closure = self.closure.clone();
        let mut receiver_1 = self.receiver_1.clone();
        let mut receiver_2 = self.receiver_2.clone();
        let handle = tokio::spawn(async move {
            let a = tokio::spawn(async move{
                let mut results = Vec::with_capacity(10);
                loop {
                    if receiver_1.changed().await.is_ok() {
                        results.push((*receiver_1.borrow()).clone());
                    }
                    if results.len()>=2{
                        return results
                    }
                }
            });

            let b = tokio::spawn(async move{
                let mut results = Vec::with_capacity(10);
                loop {
                    if receiver_2.changed().await.is_ok() {
                        results.push((*receiver_2.borrow()).clone());
                    }
                    if results.len()>=2{
                        return results
                    }
                }
            });
              return tokio::join!(a,b);
        });
        let (a,b) = handle.await.unwrap();
        sender.send((closure)(a.unwrap(),b.unwrap())).unwrap();
    }
}

impl<T, U> Drop for Multiprocessor<T, U> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort()
        }
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
        let handle =tokio::spawn(async move  {
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
        let value = "111".to_string();
        let handle = task::spawn(async move {
            tx2.send(value).await.unwrap();
            tx2.send("AA".to_string()).await.unwrap();
        });

        let handle3 = task::spawn(async move {
            tx1.send("222".to_string()).await.unwrap();
            tx1.send("BBB".to_string()).await.unwrap();
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
        let (sender_1, mut receiver_1) = watch::channel(2);
        let (sender_2, mut receiver_2) = watch::channel(2);
        let (sender_3, mut receiver_3) = watch::channel(2);

        let mut iprocessor = Multiprocessor::new(receiver_1, receiver_2, sender_3, |x, y|x.iter().sum::<i32>()+y.iter().sum::<i32>() );


        let a = tokio::spawn(async move{
            iprocessor.start().await
        });
        sender_1.send(3).unwrap();
        sender_1.send(3).unwrap();
        sender_1.send(3).unwrap();
        sender_2.send(3).unwrap();
        sender_2.send(3).unwrap();
        sender_2.send(3).unwrap();
        let b = a.await.unwrap();
        assert!(receiver_3.changed().await.is_ok());
        assert_eq!(*receiver_3.borrow(), 4);
    }
}