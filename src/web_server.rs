use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

pub fn serve(host: &Path, port: i32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, host);
    }
}

fn handle_connection(mut stream: TcpStream, host: &Path) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line.starts_with("GET /") && request_line.ends_with("HTTP/1.1") {
        let full_file_name = request_line
            .strip_prefix("GET ")
            .unwrap()
            .strip_suffix("HTTP/1.1")
            .unwrap()
            .trim();
        let mut splitted_file_name = full_file_name.split('/').collect::<Vec<&str>>();
        let last_file_with_query = splitted_file_name
            .pop()
            .unwrap()
            .split("?")
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string();
        splitted_file_name.push(&last_file_with_query);
        if !last_file_with_query.contains('.') {
            splitted_file_name.push("index.html");
        }
        let file_path_string =
            format!("{}{}", host.to_str().unwrap(), splitted_file_name.join("/"));
        println!("{}", &file_path_string);
        let file_path = Path::new(&file_path_string);
        if !file_path.exists() {
            not_found(stream);
            return;
        }

        match file_path.extension().unwrap().to_str().unwrap() {
            "html" => write_ok_text(stream, file_path),
            "css" => write_ok_text(stream, file_path),
            "txt" => write_ok_text(stream, file_path),
            "json" => write_ok_text(stream, file_path),
            "js" => write_ok_text(stream, file_path),
            "webmanifest" => write_ok_text(stream, file_path),
            "pdf" => write_ok_binary(stream, file_path),
            "png" => write_ok_binary(stream, file_path),
            "jpg" => write_ok_binary(stream, file_path),
            _ => write_not_implemented(stream, file_path.extension().unwrap().to_str().unwrap()),
        }

        return;
    }

    not_found(stream)
}

fn write_not_implemented(mut stream: TcpStream, ext: &str) {
    println!("{} not implemented", ext);
    let status_line = "HTTP/1.1 501 NOT IMPLEMENTED";
    let contents = "501 not implemented";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn write_ok_text(mut stream: TcpStream, file_path: &Path) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(&file_path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn write_ok_binary(mut stream: TcpStream, file_path: &Path) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read(&file_path).unwrap();
    let length = contents.len();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n", status_line, length);

    stream.write(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
}

fn not_found(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = "404 not found";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
