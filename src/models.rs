use actix_web::{web::Bytes, Error};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize)]
pub struct Status {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub message: String,
}

pub struct TaskStream {
    pub next: u32,
    pub buf: Vec<u8>,
}

impl futures::Stream for TaskStream {
    type Item = Result<Bytes, Error>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.next == 100_000 {
            Poll::Ready(None)
        } else {
            let res = serde_json::to_writer(
                &mut this.buf,
                &Task {
                    id: this.next,
                    name: "Coucou ceci est mon nom",
                    message: String::from(
                        "Mon message doit être un peu long pour augmenter la taille",
                    ),
                },
            );

            if let Err(e) = res {
                return Poll::Ready(Some(Err(e.into())));
            }

            this.next += 1;

            let poll = Poll::Ready(Some(Ok(Bytes::copy_from_slice(&this.buf))));

            this.buf.clear();

            poll
        }
    }
}
