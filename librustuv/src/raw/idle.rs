// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// use libc::c_void;
// use std::mem;
//
use uvll;

use raw::{Loop, Handle};
use {raw, UvResult};
// use super::{Loop, Handle};
// use green::{Callback, PausableIdleCallback};

pub struct Idle {
    handle: *mut uvll::uv_idle_t,
}

impl Idle {
    pub fn new(uv_loop: &Loop) -> UvResult<Idle> {
        unsafe {
            let handle = Handle::alloc(None::<Idle>);
        assert_eq!(unsafe {
            uvll::uv_idle_init(loop_.handle, handle)
        }, 0);
            Ok(Idle { handle: handle })
        }
    }
}

impl Handle<uvll::uv_idle_t> for Idle {
    fn uv_handle_type(_: Option<Idle>) -> uvll::uv_handle_type {
        uvll::UV_IDLE
    }
    fn raw(&self) -> *mut uvll::uv_idle_t { self.handle }
    fn from_raw(t: *mut uvll::uv_idle_t) -> Idle { Idle { handle: t } }
}

//
// impl Idle {
//     pub fn new(uv_loop: &Loop) -> Result<Idle> {
//         let handle = Handle::alloc(None::<IdleWatcher>, uvll::UV_IDLE);
//         assert_eq!(unsafe {
//             uvll::uv_idle_init(loop_.handle, handle)
//         }, 0);
//         let me = box IdleWatcher {
//             handle: handle,
//             idle_flag: false,
//             callback: cb,
//         };
//         return me.install();
//     }
//
//     pub fn onetime(loop_: &mut Loop, f: proc()) {
//         let handle = Handle::alloc(None::<IdleWatcher>, uvll::UV_IDLE);
//         unsafe {
//             assert_eq!(uvll::uv_idle_init(loop_.handle, handle), 0);
//             let data: *mut c_void = mem::transmute(box f);
//             uvll::set_data_for_uv_handle(handle, data);
//             assert_eq!(uvll::uv_idle_start(handle, onetime_cb), 0)
//         }
//
//         extern fn onetime_cb(handle: *mut uvll::uv_idle_t) {
//             unsafe {
//                 let data = uvll::get_data_for_uv_handle(handle);
//                 let f: Box<proc()> = mem::transmute(data);
//                 (*f)();
//                 assert_eq!(uvll::uv_idle_stop(handle), 0);
//                 uvll::uv_close(handle, close_cb);
//             }
//         }
//
//         extern fn close_cb(handle: *mut uvll::uv_handle_t) {
//             unsafe { uvll::free_handle(handle) }
//         }
//     }
// }
//
// impl PausableIdleCallback for IdleWatcher {
//     fn pause(&mut self) {
//         if self.idle_flag == true {
//             assert_eq!(unsafe {uvll::uv_idle_stop(self.handle) }, 0);
//             self.idle_flag = false;
//         }
//     }
//     fn resume(&mut self) {
//         if self.idle_flag == false {
//             assert_eq!(unsafe { uvll::uv_idle_start(self.handle, idle_cb) }, 0)
//             self.idle_flag = true;
//         }
//     }
// }
//
// impl Handle<uvll::uv_idle_t> for IdleWatcher {
//     fn uv_handle(&self) -> *mut uvll::uv_idle_t { self.handle }
// }
//
// extern fn idle_cb(handle: *mut uvll::uv_idle_t) {
//     let idle: &mut IdleWatcher = unsafe { Handle::from_uv_handle(&handle) };
//     idle.callback.call();
// }
//
// impl Drop for IdleWatcher {
//     fn drop(&mut self) {
//         self.pause();
//         self.close_async_();
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::mem;
//     use std::cell::RefCell;
//     use std::rc::Rc;
//     use std::rt::task::{BlockedTask, Task};
//     use std::rt::local::Local;
//
//     use green::{Callback, PausableIdleCallback};
//     use uvio::EventLoop;
//     use Loop;
//     use super::IdleWatcher;
//
//     type Chan = Rc<RefCell<(Option<BlockedTask>, uint)>>;
//
//     struct MyCallback(Rc<RefCell<(Option<BlockedTask>, uint)>>, uint);
//     impl Callback for MyCallback {
//         fn call(&mut self) {
//             let task = match *self {
//                 MyCallback(ref rc, n) => {
//                     match *rc.borrow_mut().deref_mut() {
//                         (ref mut task, ref mut val) => {
//                             *val = n;
//                             match task.take() {
//                                 Some(t) => t,
//                                 None => return
//                             }
//                         }
//                     }
//                 }
//             };
//             let _ = task.wake().map(|t| t.reawaken());
//         }
//     }
//
//     fn mk(v: uint, uv: &mut EventLoop) -> (Box<IdleWatcher>, Chan) {
//         let rc = Rc::new(RefCell::new((None, 0)));
//         let cb = box MyCallback(rc.clone(), v);
//         let cb = cb as Box<Callback>;
//         let cb = unsafe { mem::transmute(cb) };
//         let mut l = Loop::wrap(uv.uv_loop());
//         (IdleWatcher::new(&mut l, cb), rc)
//     }
//
//     fn sleep(chan: &Chan) -> uint {
//         let task: Box<Task> = Local::take();
//         task.deschedule(1, |task| {
//             match *chan.borrow_mut().deref_mut() {
//                 (ref mut slot, _) => {
//                     assert!(slot.is_none());
//                     *slot = Some(task);
//                 }
//             }
//             Ok(())
//         });
//
//         match *chan.borrow() { (_, n) => n }
//     }
//
//     test!(fn not_used() {
//         let (_idle, _chan) = mk(1, ::test::local_loop());
//     })
//
//     test!(fn smoke_test() {
//         let (mut idle, chan) = mk(1, ::test::local_loop());
//         idle.resume();
//         assert_eq!(sleep(&chan), 1);
//     })
//
//     test!(fn smoke_drop() {
//         let (mut idle, _chan) = mk(1, ::test::local_loop());
//         idle.resume();
//         fail!();
//     })
//
//     test!(fn fun_combinations_of_methods() {
//         let (mut idle, chan) = mk(1, ::test::local_loop());
//         idle.resume();
//         assert_eq!(sleep(&chan), 1);
//         idle.pause();
//         idle.resume();
//         idle.resume();
//         assert_eq!(sleep(&chan), 1);
//         idle.pause();
//         idle.pause();
//         idle.resume();
//         assert_eq!(sleep(&chan), 1);
//     })
//
//     test!(fn pause_pauses() {
//         let (mut idle1, chan1) = mk(1, ::test::local_loop());
//         let (mut idle2, chan2) = mk(2, ::test::local_loop());
//         idle2.resume();
//         assert_eq!(sleep(&chan2), 2);
//         idle2.pause();
//         idle1.resume();
//         assert_eq!(sleep(&chan1), 1);
//     })
// }
//