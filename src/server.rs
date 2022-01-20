use std::net::{TcpListener,TcpStream};
use std::io::Write;
use std::io::Read;
use std::io::Result;
use std::str;
use std::io::prelude::*;
use std::fs;
use std::{thread, time};

use crate::thread_pool::ThreadPool;


pub struct Server {
  address: &'static str
}

// Read the stream to get the request information
/*GET /index.html HTTP/1.1
Host: localhost:8080
User-Agent: curl/7.64.1
Accept: 
*/
fn read_stream(stream: &mut TcpStream) -> Vec<u8> {
  let buffer_size = 512;
  let mut request_buffer = vec![];
  // let us loop & try to read the whole request data
  let mut request_len = 0usize;
  loop {
    let mut buffer = vec![0; buffer_size];
    match stream.read(&mut buffer) {
      Ok(n) => {
        if n == 0 {
          break;
        } else {
          request_len += n;
          // we need not read more data in case we have read less data than buffer size
          if n < buffer_size {
            request_buffer.append(&mut buffer[..n].to_vec());
            break;
          } else {
            request_buffer.append(&mut buffer);
          }
        }
      }
      Err(e) => {
        println!("Error in reading stream data: {:?}", e);
        break;
      }
    }
  }
  return request_buffer;
}

// Mainly parse the first line to get the Method and Path (and Version)
fn parse_request(req: String) -> (String, String, String) {
  for (idx, line) in req.lines().enumerate() {
    if idx == 0 {
      let parts: Vec<_> = line.split(' ').collect();
      return (String::from(parts[0]), String::from(parts[1]), String::from(parts[2]));
    }
  }
  return (String::from(""),String::from(""),String::from(""))
}


// Creates the response, returns an html file from the file system
fn prepare_response(method: String, path: String) -> String {

  let (status_line, file_name) = match(method.as_str(), path.as_str()) {
    ("GET", "/index.html") => ("HTTP/1.1 200 OK", "hello.html"),
    ("GET", "/olivier.html") => { 
      thread::sleep(time::Duration::from_secs(10));
      ("HTTP/1.1 200 OK", "olivier.html") 
    },
    _ => ("HTTP/1.1 404 NotFound", "404.html"),
  };

  let contents = fs::read_to_string(file_name).unwrap();
  let response = format!(
      "{}\r\nContent-Length: {}\r\n\r\n{}",
      status_line,
      contents.len(),
      contents
  );
  return response;
}


// Hanles a single request
fn handle_client(mut stream: TcpStream) -> () {
  println!("Request");

  let request_buffer = read_stream(&mut stream);
  let request = String::from_utf8_lossy(&request_buffer);
  // println!("{}", request);

  let (method, path, _) = parse_request(request.to_string());
  
  let response = prepare_response(method, path);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
  
}

impl Server {
  pub fn new() -> Self {
    Server {
      address: "127.0.0.1:8080"
    }
  }

  pub fn serve(&self) -> Result<()> {
    let listener = TcpListener::bind(self.address)?;

    let pool = ThreadPool::new(3);

    loop {
      match listener.accept() {
        Ok((stream, socket_address)) => {
          println!("Socket address: {}", socket_address);
          pool.execute( || {
            handle_client(stream)
          });
        },
        Err(e) => return Err(e)
      };
    }
  }
}