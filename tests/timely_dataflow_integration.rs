use anyhow::{
  Result
};

use std::sync::{
  Arc, Mutex
};

use timely::{
  dataflow::{
    InputHandle,
    ProbeHandle,
    operators::{
      Input, Map, Probe, Broadcast
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
  let config = timely::Config::process(2);

  timely::execute(config, move |worker| {
    let pages = pages.clone();
    let index = worker.index();
    let mut input = InputHandle::new();
    let mut probe = ProbeHandle::new();

    worker.dataflow(|scope| {
      scope.input_from(&mut input)
        .broadcast()
        .map(move |rounds| {
          for _ in 0..rounds {

            let mut pages = pages.lock().unwrap();
            let mut handle = pages.new_handle(12);
            let mut page = pages.try_alloc(&mut handle).unwrap();

            // Allocate Handle + Page
            println!("[{}] page = {:?}", index, &page);

            let _ = page.try_read_write().unwrap();
          }
        })
        .probe_with(&mut probe);
    });

    if index == 0 {
      input.send(100000);
    }

    input.advance_to(1);
    while probe.less_than(input.time()) {
      worker.step();
    }
  }).unwrap();

  Ok(())
}