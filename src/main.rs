use std::thread;
use std::time::{Duration, Instant};

struct Request;

struct Response;

trait Processor {
    fn process(&self, req: Request) -> Response;
}

struct ProcessorA;

impl Processor for ProcessorA {
    fn process(&self, _req: Request) -> Response {
        Response {}
    }
}

struct LoggingProcessor {
    inner: Box<dyn Processor>,
}

impl LoggingProcessor {
    fn wrap(proc: Box<dyn Processor>) -> Self {
        Self { inner: proc }
    }
}

impl Processor for LoggingProcessor {
    fn process(&self, req: Request) -> Response {
        println!("LoggingProcessor: processing");
        thread::sleep(Duration::from_millis(150));
        self.inner.process(req)
    }
}

struct TracingProcessor<A> {
    inner: A,
}

impl<A: Processor> TracingProcessor<A> {
    fn wrap(inner: A) -> Self {
        Self { inner }
    }
}

impl<T: Processor> Processor for TracingProcessor<T> {
    fn process(&self, req: Request) -> Response {
        let start = Instant::now();
        let res = self.inner.process(req);
        println!(
            "Tracing Processor. time took: {}ms",
            start.elapsed().as_millis()
        );
        res
    }
}

fn main() {
    println!("Hello, world!");
    let proc = ProcessorA {};
    let dec = LoggingProcessor::wrap(Box::new(proc));
    dec.process(Request {});

    println!("---");

    let tracing = TracingProcessor::wrap(dec);
    tracing.process(Request {});
}
