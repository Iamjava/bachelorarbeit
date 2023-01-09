use std::sync::{Arc, mpsc, Mutex};
use tokio::sync::watch::{Sender, Receiver};
use tokio::sync::watch;
use tokio::task::JoinHandle;

struct Processor<T: std::clone::Clone> {
    receiver: Arc<Receiver<T>>,
    sender: Arc<Sender<T>>,
    closure: fn(T)->T,
    closure_predicate: fn(&T)->bool,
    handle: Option<JoinHandle<()>>,
}

impl<T: std::marker::Sync + std::marker::Send +std::fmt::Debug + std::clone::Clone + 'static, > Processor<T> {
    fn new(receiver: Receiver<T>, sender: Sender<T>, closure: fn(T)->T,closure_predicate: fn(&T)->bool) -> Self {
        let receiver_2 = Arc::new(receiver);
        let sender_2 = Arc::new(sender);
        Self {
            receiver:receiver_2,
            sender:sender_2,
            closure,
            closure_predicate,
            handle: None,
        }
    }

    fn start(&mut self) {
        let (sender,mut receiver) = (self.sender.clone(),self.receiver.as_ref().clone());
        let closure = self.closure.clone();
        let predicate = self.closure_predicate.clone();
        let handle = tokio::spawn(async move {
            while receiver.changed().await.is_ok() {
                let val = receiver.borrow().clone();
                let processed = (closure)(val.clone());
                print!("processed {:?}",&processed);
                if (predicate)(&val){
                    sender.as_ref().send(processed).unwrap();
                }
            }
        });

        self.handle = Some(handle)
    }

    fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

impl<T: std::clone::Clone> Drop for Processor<T> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}


mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test1() {
        let (tx_orig, mut rx_struct) = watch::channel(1);
        let (tx_struct, mut rx_struct2) = watch::channel(2);
        let (tx_struct2, mut receiver) = watch::channel(3);

        let mut processor_1 = Processor::new(rx_struct, tx_struct, |x| x +1 , |x| true);
        let mut processor_2 = Processor::new(rx_struct2, tx_struct2, |x| x - 1,|x|*x>0);
        processor_1.start();
        processor_2.start();
        //processor_1.start();
        //processor_1.stop();

        //let mut processor_2 = Processor::new(receiver_2, sender_1.clone(), |x| {print!("{:?}",x);x*2});
        //processor_2.start();

        tx_orig.send(3 as u16).unwrap();
        assert!(receiver.changed().await.is_ok());
        assert_eq!(*receiver.borrow(), 3);
        tx_orig.send(1 as u16).unwrap();
        assert!(receiver.changed().await.is_ok());
        assert_eq!(*receiver.borrow(), 1);
        tx_orig.send(2 as u16).unwrap();
        //assert_eq!(receiver_2.recv().await.unwrap(),2);
    }

}