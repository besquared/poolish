use anyhow::{
  Result
};

use std::sync::{
  Arc,
  atomic::{
    AtomicUsize,
    Ordering
  }
};

use timely::{
  Config,
  dataflow::{
    InputHandle,
    ProbeHandle,
    operators::{
      Input, Map, Probe
    }
  }
};

use vex_pages::PageManager;

const POOL_SIZE: usize = usize::pow(2, 31);

fn make_page_manager() -> Result<Arc<PageManager>> {
  Ok(Arc::new(PageManager::try_new(POOL_SIZE)?))
}

#[test]
fn allocates_pages_in_ops() -> Result<()> {
  let page_size = 2048u32;
  let thread_pages = 1usize;
  let thread_count = 2usize;

  let pages = make_page_manager()?;
  let page_count = Arc::new(AtomicUsize::from(0));

  // Clones for worker
  let worker_pages = pages.clone();
  let worker_page_size = page_size.clone();
  let worker_page_count = page_count.clone();
  let worker_thread_pages = thread_pages.clone();

  timely::execute(Config::process(thread_count), move |worker| {
    let mut input = InputHandle::new();
    let mut probe = ProbeHandle::new();

    // Clones for dataflow
    let worker_pages = worker_pages.clone();
    let worker_page_size = worker_page_size.clone();
    let worker_page_count = worker_page_count.clone();
    let worker_thread_pages = worker_thread_pages.clone();

    worker.dataflow(|scope| {
      scope.input_from(&mut input)
        .map(move |rounds| {
          for _ in 0..rounds {
            match worker_pages.try_alloc(worker_page_size) {
              Err(err) => panic!("{}", err),
              Ok(mut page) => {
                worker_pages.try_free(page).unwrap();
                worker_page_count.fetch_add(1, Ordering::SeqCst);
              }
            }
          }
        })
        .probe_with(&mut probe);
    });

    // Add input to all workers
    input.send(worker_thread_pages);

    input.advance_to(1);
    while probe.less_than(input.time()) {
      worker.step();
    }
  }).unwrap();

  // Make sure we got the right number of pages
  let page_count = page_count.load(Ordering::Acquire);
  assert_eq!(page_count, thread_count * thread_pages);

  Ok(())
}