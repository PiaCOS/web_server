use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener_res = TcpListener::bind("challenge01.root-me.org:52002"); // .unwrap();
    let listener = match listener_res {
        Ok(l) => Some(l),
        Err(e) => {println!("{}", e); None},
    };
 
    for stream in listener.unwrap().incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    
    // Only gets the first line
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{}", request_line);

    // let (status_line, filename) = match request_line.as_str() { 
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    //     "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "about.html"),
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    // };

    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();

    // // The \r\n... is the CRLF (carriage return and line feed)
    // let response = format!(
    //     "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    // );
    
    // stream.write_all(response.as_bytes()).unwrap();
}
