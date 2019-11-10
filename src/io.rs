use std::pin::Pin;

use eff::{Context, Poll};

pub trait AsyncRead {
    type Error;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &Context,
        buf: &mut [u8],
    ) -> Poll<Result<u8, Self::Error>, !>;
}

pub trait AsyncWrite {
    type Error;

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &Context,
        buf: &mut [u8],
    ) -> Poll<Result<u8, Self::Error>, !>;
    fn poll_flush(self: Pin<&mut Self>, cx: &Context) -> Poll<Result<(), Self::Error>, !>;
    fn poll_close(self: Pin<&mut Self>, cx: &Context) -> Poll<Result<(), Self::Error>, !>;
}
