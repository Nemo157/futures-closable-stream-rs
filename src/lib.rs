#![cfg_attr(feature = "nightly", feature(arbitrary_self_types))]
#![cfg_attr(feature = "nightly", feature(in_band_lifetimes))]

#![no_std]

macro_rules! if_nightly {
    ($($item:item)*) => {
        $(#[cfg(feature = "nightly")] $item)*
    }
}

extern crate futures_core;

use futures_core::{task, Future, Poll, Stream};

pub trait ClosableStream: Stream {
    fn poll_close(&mut self, cx: &mut task::Context) -> Poll<(), Self::Error>;
}

pub trait ClosableStreamExt {
    fn close(self) -> Close<Self> where Self: Sized {
        Close { stream: self }
    }
}

pub struct Close<S> {
    stream: S,
}

impl<S: ClosableStream> Future for Close<S> {
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        self.stream.poll_close(cx)
    }
}

impl<S: ClosableStream> ClosableStreamExt for S {
}

if_nightly! {
    extern crate pin_api;
    extern crate futures_stable;

    use pin_api::mem::Pin;
    use futures_stable::StableStream;

    pub trait ClosableStableStream: StableStream {
        fn poll_close(self: Pin<Self>, cx: &mut task::Context) -> Poll<(), Self::Error>;
    }

    pub trait ClosableStableStreamExt {
        fn close(self: Pin<'a, Self>) -> CloseStable<'a, Self> {
            CloseStable { stream: self }
        }
    }

    pub struct CloseStable<'a, S: 'a + ?Sized> {
        stream: Pin<'a, S>,
    }

    impl<'a, S: ClosableStableStream> Future for CloseStable<'a, S> {
        type Item = ();
        type Error = S::Error;

        fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
            Pin::borrow(&mut self.stream).poll_close(cx)
        }
    }

    impl<S: ClosableStableStream> ClosableStableStreamExt for S {
    }
}
