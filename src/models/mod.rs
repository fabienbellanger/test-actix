pub mod auth;
pub mod user;

use actix_web::{web::Bytes, Error};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub age: u32,
}

#[derive(Deserialize)]
pub struct Query {
    pub username: Option<String>,
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
    pub number: u32,
    pub next: u32,
    pub buf: Vec<u8>,
}

impl futures::Stream for TaskStream {
    type Item = Result<Bytes, Error>;

    // TODO: Ne fonctionne pas très bien si this.number = 0
    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.next == this.number {
            // Stop stream
            Poll::Ready(None)
        } else {
            // Array start
            if this.next == 0 {
                for v in b"[" {
                    this.buf.push(*v);
                }
            }

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

            if this.next < this.number {
                // Comma between tasks
                for v in b"," {
                    this.buf.push(*v);
                }
            } else {
                // Array end
                for v in b"]" {
                    this.buf.push(*v);
                }
            }

            let poll = Poll::Ready(Some(Ok(Bytes::copy_from_slice(&this.buf))));

            this.buf.clear();

            poll
        }
    }
}
