
use std::sync::{Arc, Mutex};
use tokio::sync::watch;
use tokio::task::JoinHandle;

struct Multiprocessor<T, U> {
    receiver_1: watch::Receiver<T>,
    receiver_2: watch::Receiver<U>,
    sender: Arc<watch::Sender<T>>,
    closure: fn(Vec<T>,Vec<U>) -> (T),
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

    async fn start(&mut self) {
        let sender = self.sender.clone();
        let closure = self.closure.clone();

        let mut receiver_1 = self.receiver_1.clone();
        let mut receiver_2 = self.receiver_2.clone();
        let handle = tokio::spawn(async move {
            let mut results = None;
            let mut results2 =None;
            loop {
                tokio::select! {
                    Ok(s) = receiver_1.changed() => results = Some((*receiver_1.borrow()).clone()),
                    Ok(s) = receiver_2.changed() => results2 = Some((*receiver_2.borrow()).clone()),
                }
                if results.is_some() && results2.is_some(){
                    sender.send(results.clone().unwrap()).unwrap();
                }
            }
        });

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
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test2() {
        let (sender_1, mut receiver_1) = watch::channel(2);
        let (sender_2, mut receiver_2) = watch::channel(2);
        let ( sender_3, mut receiver_3) = watch::channel(2);

        let mut iprocessor = Multiprocessor::new(receiver_1, receiver_2, sender_3, |x, y|33 );


        let a = iprocessor.start().await;
        sender_1.send(3).unwrap();
        sender_2.send(3).unwrap();
        assert!(receiver_3.changed().await.is_ok());
        assert_eq!(*receiver_3.borrow(), 4);
    }
}