use std::fs::OpenOptions;
use std::io::Write;
use std::marker::PhantomData;

use capnp::message::{Allocator, Builder};
use capnp::serialize::{read_message, write_message};
use failure::Error;

pub struct QueueWriter<T: Allocator> {
    w: Box<Write>,
    p: PhantomData<T>
}

impl<T: Allocator> QueueWriter<T> {
    fn new(w: Box<Write>) -> QueueWriter<T> {
        QueueWriter{
            w: w,
            p: PhantomData
        }
    }
    
    fn from_file(filename: &str) -> Result<QueueWriter<T>, Error> {
        Ok(QueueWriter{
            w: Box::new(OpenOptions::new().create(true).append(true).open(filename)?),
            p: PhantomData
        })
    }

    fn put(&mut self, msg: Builder<T>) {
        write_message(&mut self.w, &msg);
    }
}

pub struct QueueReaderr<T: Allocator> {
    w: Box<Write>,
    p: PhantomData<T>
}

impl<T: Allocator> QueueReaderr<T> {
    fn new(w: Box<Write>) -> QueueWriter<T> {
        QueueWriter{
            w: w,
            p: PhantomData
        }
    }
    
    fn from_file(filename: &str) -> Result<QueueWriter<T>, Error> {
        Ok(QueueWriter{
            w: Box::new(OpenOptions::new().create(true).append(true).open(filename)?),
            p: PhantomData
        })
    }

    fn get(&mut self, msg: Builder<T>) {
        write_message(&mut self.w, &msg);
    }
}
