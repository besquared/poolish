use anyhow::{
  Result
};

use std::sync::{
  Arc, Mutex,
  atomic::{
    AtomicUsize, Ordering
  }
};

use timely::{
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

fn make_page_manager() -> Result<Arc<Mutex<PageManager>>> {
  Ok(Arc::new(Mutex::new(PageManager::try_new(POOL_SIZE)?)))
}

#[test]
fn allocates_pages_in_ops() -> Result<()> {
  let pages = make_page_manager()?;
  let page_count = Arc::new(AtomicUsize::from(0));

  let page_size = 2048u32;
  let thread_count = 2usize;
  let pages_per_thread = 1usize;
  let config = timely::Config::process(thread_count);

  timely::execute(config, move |worker| {
    let pages_ref = pages.clone();
    let page_count_ref = page_count.clone();

    let mut input = InputHandle::new();
    let mut probe = ProbeHandle::new();

    worker.dataflow(|scope| {
      scope.input_from(&mut input)
        .map(move |rounds| {
          for _ in 0..rounds {
            let mut pages = pages_ref.lock().unwrap();
            let mut handle = pages.try_new_handle(page_size).unwrap();

            pages.try_alloc(&mut handle).unwrap();
            page_count_ref.fetch_add(1, Ordering::SeqCst);
          }
        })
        .probe_with(&mut probe);
    });

    // Add input to all workers
    input.send(pages_per_thread);

    input.advance_to(1);
    while probe.less_than(input.time()) {
      worker.step();
    }

    // Make sure we got the right number of pages
    let page_count = page_count.load(Ordering::Acquire);
    assert_eq!(page_count, thread_count * pages_per_thread);
  }).unwrap();

  Ok(())
}