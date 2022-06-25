use std::env;
use std::error::Error;
use suppaftp::FtpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let host = args.get(1).expect("failed to get host").to_string();
    let port = args.get(2).expect("failed to get port").to_string();
    let user = args.get(3).expect("failed to get user").to_string();
    let passwd = args.get(4).expect("failed to get passwd").to_string();
    let root = args.get(5).expect("failed to get root").to_string();
    let mut ftp_stream = FtpStream::connect(format!("{}:{}", host, port))?;
    let _ = ftp_stream.login(user, passwd)?;
    walk(&mut ftp_stream, &root)?;
    let _ = ftp_stream.quit();
    Ok(())
}

fn walk(ftp_stream: &mut FtpStream, root: &str) -> Result<(), Box<dyn Error>> {
    let files = ftp_stream.list(Some(&root))?;
    for file_info in files.iter() {
        let mut sp = file_info.split_whitespace();
        let file_type = sp.next().expect("failed to get file-type");
        let file_name = sp.last().expect("failed to get filename");
        if file_type.is_empty() || file_name.is_empty() || file_name == "." || file_name == ".." { continue; }
        if file_type.starts_with("d") { // ^drwx
            let root = format!("{}/{}", root, file_name);
            walk(ftp_stream, &root)?;
            continue;
        }
        println!("{}/{}", root, file_name);
    }
    Ok(())
}
