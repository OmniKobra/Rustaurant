use super::Category;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::{Duration, Sleep, sleep};

#[derive(Default, PartialEq, Eq)]
pub enum OrderStatus {
    #[default]
    Idle,
    Preparing,
    Served,
    Complete,
}
#[derive(Default)]
pub struct Order<'a> {
    pub name: &'a str,
    pub count: u32,
    pub status: OrderStatus,
    pub category: Category,
    pub customer: u32,
    pub idle2prep: Option<Pin<Box<Sleep>>>,
    pub prep2serv: Option<Pin<Box<Sleep>>>,
    pub serv2eat: Option<Pin<Box<Sleep>>>,
}

impl<'a> Future for Order<'a> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.status == OrderStatus::Complete {
            return Poll::Ready(());
        }
        let handler = |s: &mut Option<Pin<Box<Sleep>>>| {
            if s.is_none() {
                *s = Some(Box::pin(sleep(Duration::from_secs(1))));
            }
        };
        handler(&mut self.idle2prep);
        let fut = self.idle2prep.as_mut().unwrap().as_mut();
        match fut.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => {
                self.status = OrderStatus::Preparing;
                handler(&mut self.prep2serv);
                let fut = self.prep2serv.as_mut().unwrap().as_mut();
                match fut.poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(_) => {
                        self.status = OrderStatus::Served;
                        handler(&mut self.serv2eat);
                        let fut = self.serv2eat.as_mut().unwrap().as_mut();
                        match fut.poll(cx) {
                            Poll::Pending => Poll::Pending,
                            Poll::Ready(_) => {
                                self.status = OrderStatus::Complete;
                                Poll::Ready(())
                            }
                        }
                    }
                }
            }
        }
    }
}
