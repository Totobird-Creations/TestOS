use core::{
    pin::Pin,
    task::{
        Poll,
        Context
    }
};

use conquer_once::spin::OnceCell;
use crossbeam_queue::{
    ArrayQueue,
    PopError
};
use futures_util::{
    stream::{
        Stream,
        StreamExt
    },
    task::AtomicWaker
};
use pc_keyboard::{
    layouts::Us104Key,
    DecodedKey,
    HandleControl,
    Keyboard,
    ScancodeSet1
};

use crate::vga;

static SCANCODE_QUEUE : OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER          : AtomicWaker              = AtomicWaker::new();


struct ScancodeStream;
impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new called more than once.");
        return ScancodeStream;
    }
}
impl Stream for ScancodeStream {
    type Item = u8;
    fn poll_next(self : Pin<&mut Self>, context : &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("Scancode queue not initialised.");
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }
        WAKER.register(&context.waker());
        match (queue.pop()) {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            },
            Err(PopError) => Poll::Pending
        }
    }
}


pub fn add_scancode(scancode : u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            vga::warn!("scancode queue full.");
        } else {
            WAKER.wake();
        }
    } else {
        vga::warn!("scancode queue uninitialised.");
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard  = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::Ignore);
    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(event) {
                match (key) {
                    DecodedKey::Unicode (character ) => vga::print!("{}", character),
                    DecodedKey::RawKey  (key       ) => vga::print!("{:?}", key)
                }
            }
        }
    }
}
