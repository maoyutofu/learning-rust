use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

// 增加健壮性 也就是鲁棒性
impl Drop for ThreadPool {
	fn drop(&mut self) {
		// for worker in &mut self.workers {
		// 	println!("Shutting down worker {}", worker.id);
		// 	if let Some(thread) = worker.thread.take() {
		// 		thread.join().unwrap();
		// 	}
		// }
		println!("Sending terminate message to all workers.");

		for _ in &mut self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shutting down all workers.");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)()
	}
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
	NewJob(Job),
	Terminate,
}

impl Worker {
	fn new(id: usize, recevier: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				// let job = recevier.lock().unwrap().recv().unwrap();
				let message = recevier.lock().unwrap().recv().unwrap();
				println!("Worker {} got a jobl; executing.", id);
				// (*job)();
				// job.call_box();
				match message {
					Message::NewJob(job) => {
						println!("Worker {} got a job; executing.", id);
						job.call_box();
					},
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					},
				}
			}
		});
		Worker {
			id,
			thread: Some(thread),
		}
	}
}

struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));
		
		let mut workers = Vec::with_capacity(size);
		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)))
		}
		ThreadPool {
			workers,
			sender,
		}
	}

	fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
		let job = Box::new(f);
		// self.sender.send(job).unwrap();
		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 512];
	stream.read(&mut buf).unwrap();
	println!("Request: {}", String::from_utf8_lossy(&buf));

	let index = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";

	let (status_line, filename) = if buf.starts_with(index) {
		("HTTP/1.1 200 OK\r\n\r\n", "index.html")
	}else if buf.starts_with(sleep){
		thread::sleep(Duration::from_secs(5));
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
	} else {
		("HTTP/1.1 404 Not Found\r\n\r\n", "404.html")
	};

	let mut file = File::open(filename).unwrap();

	let mut str_buf = String::new();
	file.read_to_string(&mut str_buf).unwrap();

	let response = format!("{}{}", status_line, str_buf);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
	let pool = ThreadPool::new(10);
	let mut counter = 0;
	for stream in listener.incoming() {
		// 达到两个请求后 就退出
		if counter == 2 {
			println!("Shutting down.");
			break;
		}
		counter += 1;

		let mut stream = stream.unwrap();
		println!("Connection established!");
		
		// thread::spawn(|| {
		// 	handle_connection(stream);
		// });

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}