//! An [HttpBody](https://docs.rs/hyper/0.14.11/hyper/body/trait.HttpBody.html) implementation with efficient streaming support for the Rust HTTP library [hyper](https://hyper.rs/).
//!
//! # Motivation
//!
//! The existing [Body](https://docs.rs/hyper/0.14.11/hyper/body/struct.Body.html) type in [hyper](https://hyper.rs/) uses [Bytes](https://docs.rs/bytes/0.5.4/bytes/struct.Bytes.html)
//! as streaming chunk. Hence, a lot of buffer allocation and de-allocation happen during the real-time large data streaming because of the [Bytes](https://docs.rs/bytes/0.5.4/bytes/struct.Bytes.html) type.
//! Therefore, `StreamBody` comes to tackle this kind of situation. The `StreamBody` implements [HttpBody](https://docs.rs/hyper/0.14.11/hyper/body/trait.HttpBody.html) and uses `&[u8]`
//! slice as the streaming chunk, so it is possible to use the same buffer without allocating a new one; hence it overcomes any allocation/de-allocation overhead.
//!
//! Also, the [channel()](https://docs.rs/hyper/0.14.11/hyper/body/struct.Body.html#method.channel) method in hyper [Body](https://docs.rs/hyper/0.14.11/hyper/body/struct.Body.html) returns
//! a pair of a [Sender](https://docs.rs/hyper/0.14.11/hyper/body/struct.Sender.html) and a [Body](https://docs.rs/hyper/0.14.11/hyper/body/struct.Body.html).
//! Here, the [Sender](https://docs.rs/hyper/0.14.11/hyper/body/struct.Sender.html) accepts [Bytes](https://docs.rs/bytes/0.5.4/bytes/struct.Bytes.html) as a data chunk which again
//! creates allocation/de-allocation overhead.
//! To solve this, `StreamBody` has a method named `StreamBody::channel()` which returns a pair of an [AsyncWrite](https://docs.rs/tokio/0.2.16/tokio/io/trait.AsyncWrite.html) and the `StreamBody`
//! itself. As the [AsyncWrite](https://docs.rs/tokio/0.2.16/tokio/io/trait.AsyncWrite.html) accepts `&[u8]` instead of [Bytes](https://docs.rs/bytes/0.5.4/bytes/struct.Bytes.html), there will
//! be no allocation/de-allocation overhead.


pub use self::body::StreamBody;
pub use self::data::StreamData;

mod body;
mod data;
mod state;
