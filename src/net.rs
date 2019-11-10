use std::marker::PhantomData;
use std::net::ToSocketAddrs;

use crate::io::{AsyncRead, AsyncWrite};
use eff::Effect;

pub trait TcpStream: AsyncRead + AsyncWrite {
    // TODO: add tcp methods
}

#[derive(Debug)]
pub struct Connect<T, A> {
    addrs: A,
    phantom: PhantomData<T>,
}

impl<T, A> Connect<T, A> {
    pub fn new(addrs: A) -> Self {
        Connect {
            addrs,
            phantom: PhantomData,
        }
    }
}

impl<T, A> Effect for Connect<T, A>
where
    T: TcpStream,
    A: ToSocketAddrs,
{
    type Output = T;
}
