use js_sys::Promise;
use std::sync::mpsc;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{DedicatedWorkerGlobalScope, Worker, WorkerOptions, WorkerType};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn setup(n: usize) -> Promise {
    console_error_panic_hook::set_once();

    let mut opts = WorkerOptions::new();
    opts.type_(WorkerType::Module);
    let mut rxs = vec![];
    for i in 0..n {
        log(&*format!("Starting worker {}", i));
        opts.name(&*i.to_string());
        let worker = Worker::new_with_options("./worker.js", &opts).unwrap();
        let arr = js_sys::Array::new();
        arr.push(&wasm_bindgen::module());
        arr.push(&wasm_bindgen::memory());
        let (tx, rx) = mpsc::channel::<String>();
        let ptr = Box::into_raw(Box::new(tx));
        arr.push(&JsValue::from(ptr as u32));

        worker.post_message(&arr).unwrap();
        rxs.push((rx, worker));
    }
    wasm_bindgen_futures::future_to_promise(async move {
        // This is a bit of a poor example, as the js event loop must not be blocked. Let's give
        // the workers some time to start up and send their message..
        futures_timer::Delay::new(Duration::from_secs(5)).await;
        for (i, (rx, worker)) in rxs.into_iter().enumerate() {
            let x = rx.recv().unwrap();
            assert!(x == format!("Hello from Worker {}", i));
            log(&*format!("Terminating worker {}", i));
            worker.terminate();
        }
        Ok("".into())
    })
}

#[wasm_bindgen]
pub fn entry(ptr: u32) {
    let tx = unsafe { Box::from_raw(ptr as *mut mpsc::Sender<String>) };
    let name = js_sys::global()
        .unchecked_into::<DedicatedWorkerGlobalScope>()
        .name();
    tx.send(format!("Hello from Worker {}", name)).unwrap();
}
