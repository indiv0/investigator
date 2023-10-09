use crate::prelude::*;

use find_files_core::channel;
use investigator::Hasher as _;
use std::io::Read as _;
use std::io::Write as _;



// ==============
// === Export ===
// ==============

pub mod walkdir;



// ===============
// === Prelude ===
// ===============

mod prelude {
    // Re-exports for in-crate use.
    pub(crate) use find_files_core::prelude::*;
    pub(crate) use find_files_core::shell::app::Event;
    pub(crate) use find_files_core::channel::Sender;
    pub(crate) use find_files_core::hash::HashResponse;
    pub(crate) use find_files_core::shell::core::Core;
    pub(crate) use find_files_core::shell::core::Effect;
    pub(crate) use find_files_core::walkdir::WalkdirResponse;
    pub(crate) use core::pin::Pin;
    pub(crate) use core::fmt;
    pub(crate) use core::fmt::Debug;
    pub(crate) use core::fmt::Formatter;
    pub(crate) use core::future::Future;
    pub(crate) use std::fs;
    pub(crate) use std::io;
    pub(crate) use std::path::PathBuf;
    pub(crate) use std::sync::Arc;
}



// ===============
// === Spawner ===
// ===============

#[derive(Clone)]
#[must_use]
struct Spawner {
    inner: Arc<dyn Fn(Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>) + Send + Sync>,
}

static_assertions::assert_impl_all!(Spawner: Clone, Send);

impl Spawner {
    fn spawn(&self, future: impl Future<Output = Result<(), ()>> + Send + 'static) {
        let future = Box::pin(future) as _;
        (self.inner)(future)
    }
}

impl Debug for Spawner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spawner").finish()
    }
}



// ======================
// === process_effect ===
// ======================

fn process_effect(
    core: &Arc<Core>,
    spawner: &Spawner,
    effect: Effect,
    tx: &Arc<Sender<Effect>>,
) -> Result<(), ()> {
    // FIXME [NP]: Clean
    //println!("Shell process effect: {effect:?}.");
    match effect {
        Effect::KeyValue(mut request) => match request.operation {
            KeyValueOperation::Read(ref key) => {
                spawner.spawn({
                    let core = core.clone();
                    let spawner = spawner.clone();
                    let tx = tx.clone();
                    let key = key.clone();

                    async move {
                        let bytes = read_state(&key);
                        let bytes = bytes.map_err(|e| eprintln!("{e:?}."));
                        let response = KeyValueOutput::Read(bytes);

                        for effect in core.resolve(&mut request, response) {
                            process_effect(&core, &spawner, effect, &tx)?;
                        }
                        Result::<(), ()>::Ok(())
                    }
                });
            },
            KeyValueOperation::Write(ref key, ref value) => {
                spawner.spawn({
                    let core = core.clone();
                    let spawner = spawner.clone();
                    let tx = tx.clone();
                    let key = key.clone();
                    let value = value.clone();

                    async move {
                        let result = write_state(&key, &value);
                        let result = result.map_err(|e| eprintln!("{e:?}."));
                        let response = KeyValueOutput::Write(result);

                        for effect in core.resolve(&mut request, response) {
                            process_effect(&core, &spawner, effect, &tx)?;
                        }
                        Result::<(), ()>::Ok(())
                    }
                });
            },
        },
        Effect::Walkdir(mut request) => {
            let path = request.operation.path.as_path();
            let entries = walkdir::walkdir(path);
            let entries = entries.map(|e| e.path().to_path_buf());
            let entries = entries.collect();
            let response = WalkdirResponse { paths: entries };

            for effect in core.resolve(&mut request, response) {
                process_effect(&core, &spawner, effect, &tx)?;
            }
        },
        Effect::Render(_) => tx.send(effect),
        Effect::Hash(mut request) => {
            spawner.spawn({
                let core = core.clone();
                let spawner = spawner.clone();
                let tx = tx.clone();

                async move {
                    let path = &request.operation.path;
                    let mut file = fs::File::open(path).expect("Open file");
                    // FIXME [NP]: Don't depend on the whole `investigator` crate?
                    let mut hasher = investigator::T1ha2::default();
                    investigator::copy_wide(&mut file, &mut hasher).expect("Hash file");
                    let hash = hasher.finish().to_vec();
                    let hash = hex::encode(hash);
                    let response = HashResponse { hash };

                    for effect in core.resolve(&mut request, response) {
                        process_effect(&core, &spawner, effect, &tx)?;
                    }
                    Result::<(), ()>::Ok(())
                }
            });
        },
    }
    Ok(())
}

fn read_state(key: &str) -> io::Result<Vec<u8>> {
    // FIXME [NP]: Make this async
    let path = format!(".find_files.{key}");
    let mut f = fs::File::open(path)?;
    let mut buf: Vec<u8> = vec![];

    // FIXME [NP]: Make this async
    f.read_to_end(&mut buf)?;

    Ok(buf)
}

fn write_state(key: &str, value: &[u8]) -> io::Result<()> {
    // FIXME [NP]: Make this async
    let path = format!(".find_files.{key}");
    let mut f = fs::File::create(path)?;

    // FIXME [NP]: Make this async
    f.write_all(&value)?;

    Ok(())
}



// ==============
// === update ===
// ==============

fn update(
    core: &Arc<Core>,
    spawner: &Spawner,
    event: Event,
    tx: &Arc<Sender<Effect>>,
) -> Result<(), ()> {
    println!("Shell update: {event:?}.");
    for effect in core.process_event(event) {
        process_effect(core, spawner, effect, &tx)?;
    }
    Ok(())
}




// =============
// === Shell ===
// =============

#[must_use]
pub struct Shell {
    pub core: Arc<Core>,
    spawner: Spawner,
    render_tx: Sender<Effect>,
}

impl Shell {
    pub fn new(
        spawner: impl Fn(Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>) + Send + Sync + 'static,
        render_tx: Sender<Effect>,
    ) -> Self {
        let core = Core::default();
        let core = Arc::new(core);
        let spawner = Arc::new(spawner);
        let spawner = Spawner { inner: spawner };
        Self { core, spawner, render_tx }
    }

    pub fn run(&self, events: Vec<Event>) -> Result<(), ()> {
        println!("Shell::run: {events:?}.");
        let render_tx = Arc::new(self.render_tx.clone());
        for event in events {
            update(&self.core, &self.spawner, event, &render_tx.clone())?;
        }

        Ok(())
    }

}

impl Debug for Shell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shell").finish()
    }
}



// ===========
// === run ===
// ===========

pub fn run(
    path: impl Into<PathBuf>,
    spawner: impl Fn(Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>) + Send + Sync + 'static,
) -> Vec<PathBuf> {
    let path = path.into();
    let (render_tx, render_rx) = channel::channel();
    let shell = Shell::new(spawner, render_tx);
    let event = Event::Walkdir(path);
    let events = vec![event];
    shell.run(events).expect("Run shell");
    // Wait for core to settle.
    // We could process the render effect(s) here but we do it once at the end, instead.
    while let Some(_effect) = render_rx.recv() {}
    let model = shell.core.model.read();
    // FIXME [NP]: Replace instead of cloning.
    let paths = model.paths.iter();
    let paths = paths.map(PathBuf::from);
    paths.collect()
}
