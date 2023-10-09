#![feature(io_error_more)]
#![feature(negative_impls)]
#![feature(assert_matches)]
use crate::prelude::*;

use investigator::Hasher as _;



// ==============
// === Export ===
// ==============

pub mod channel;
mod context;
pub mod executor;
pub mod hash;
pub mod key_value;
mod mutex;
mod path;
mod render;
pub mod request;
mod resolve;
mod rw_lock;
pub mod shell;
mod testing;
pub mod walkdir;



// ===============
// === Prelude ===
// ===============

pub mod prelude {
    // Re-exports for in-crate use.
    pub(crate) use crate::BoxFuture;
    pub(crate) use crate::BoxFutureLifetime;
    pub(crate) use crate::Unclone;
    pub(crate) use crate::Unsync;
    pub(crate) use crate::channel::Receiver;
    pub(crate) use crate::channel::Sender;
    pub(crate) use crate::context::CapabilityContext;
    pub(crate) use crate::context::ProtoContext;
    pub(crate) use crate::context::WithContext;
    pub(crate) use crate::mutex::Mutex;
    pub(crate) use crate::rw_lock::RwLock;
    pub(crate) use crate::shell::app::assert_path_rules;
    pub(crate) use crate::shell::app::App;
    pub(crate) use crate::shell::app::Capabilities;
    pub(crate) use crate::shell::app::lines::Lines;
    pub(crate) use core::future::Future;
    pub(crate) use core::fmt;
    pub(crate) use core::fmt::Debug;
    pub(crate) use core::fmt::Display;
    pub(crate) use core::fmt::Formatter;
    pub(crate) use core::ops::Deref;
    pub(crate) use core::pin::Pin;
    pub(crate) use core::task::Context;
    pub(crate) use core::task::Poll;
    pub(crate) use miniserde::Deserialize;
    pub(crate) use miniserde::Serialize;
    pub(crate) use std::assert_matches::assert_matches;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::collections::HashMap;
    pub(crate) use std::marker::PhantomData;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
    pub(crate) use std::rc::Rc;
    pub(crate) use std::sync;
    pub(crate) use std::sync::mpsc;
    pub(crate) use std::sync::Arc;
    pub(crate) use std::task::Waker;
    // Re-exports for external use.
    pub use crate::key_value::protocol::KeyValueOperation;
    pub use crate::key_value::protocol::KeyValueOutput;
    pub use crate::shell::app::Event;
    pub use crate::shell::app::Model;
    pub use crate::shell::core::Effect;
    pub use crate::testing::AppTester;
    pub use crate::walkdir::protocol::WalkdirRequest;
    pub use crate::hash::protocol::HashResponse;
}



// =================
// === BoxFuture ===
// =================

type BoxFuture<T = ()> = Pin<Box<dyn Future<Output = T> + Send>>;
type BoxFutureLifetime<'a, T = ()> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;



// ===============
// === Unclone ===
// ===============

#[derive(Debug, Default)]
struct Unclone;

static_assertions::assert_not_impl_all!(Unclone: Clone);

impl context::Operation for Unclone {
    type Output = Unclone;
}



// ==============
// === Unsync ===
// ==============

#[derive(Clone, Copy, Debug, Default)]
struct Unsync;

static_assertions::assert_not_impl_all!(Unsync: Sync);

impl context::Operation for Unsync {
    type Output = Unsync;
}

impl !Sync for Unsync {}



// ==================
// === hash_bytes ===
// ==================

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = investigator::T1ha2::default();
    investigator::copy_wide(&mut &bytes[..], &mut hasher).unwrap();
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}
