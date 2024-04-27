use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::*;

pub struct Calc {
    inner: Arc<Mutex<Inner>>,

    sender: UnboundedSender<(String, i32)>,
}

struct Inner {
    result: i32,
}

impl Clone for Calc {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            sender: self.sender.clone(),
        }
    }
}

impl Calc {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded_channel::<(String, i32)>();
        let inner = Arc::new(Mutex::new(Inner {
            result: 0,
        }));

        let calc = Self {
            inner,
            sender
        };

        tokio::spawn(calc.clone().process(receiver));
        calc
    }

    pub fn send(&mut self, message: (String, i32)) {
        self.sender.send(message).ok();
    }

    pub fn result(&self) -> i32 {
        if let Ok(inner) = self.inner.lock() {
            return inner.result;
        }
        return 0;
    }

    fn add(&mut self, v: i32) {
        if let Ok(mut inner) = self.inner.lock() {
            println!("add {} before: {}", v, inner.result);
            inner.result += v;
            println!("add after: {}", inner.result);
        }
    }

    async fn process(mut self, mut receiver: UnboundedReceiver<(String, i32)>) {
        while let Some((cmd, v)) = receiver.recv().await {
            if &cmd == "add" {
                self.add(v);
            }
        }
    }
}