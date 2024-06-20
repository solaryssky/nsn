use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs::File;
use chrono::Duration;
use ssh2::Session;
use std::path::Path;
use std::net::SocketAddr;
use suppaftp::FtpStream;
use log::info;



pub fn sftp_download(host_port: &str, user: &str, password: &str, src: &str, cp_full_path: &str){
    info!("Trying to connect {}", &host_port);
    let tcp = TcpStream::connect(host_port).unwrap();
    info!("Connected OK {}", &host_port);
    let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            sess.handshake().unwrap();
            sess.userauth_password(user, password).unwrap();
    let mut contents_tts:Vec<u8> = Vec::new();
    let mut contents_ttc:Vec<u8> = Vec::new();
    let sftp = sess.sftp().unwrap();
    
    let mut stream_tts = sftp.open(Path::new(&(src.to_owned() + "/TTSCOF00.IMG"))).unwrap();
    info!("Download TTSCOF00.IMG from {}", &host_port); 
            stream_tts.read_to_end(&mut contents_tts).unwrap();
            let _ = std::fs::write(cp_full_path.to_owned() + "/TTSCOF00.IMG", &contents_tts);
    let mut stream_ttc = sftp.open(Path::new(&(src.to_owned() + "/TTTCOF00.IMG"))).unwrap();
    info!("Download TTTCOF00.IMG from {}", &host_port);
            stream_ttc.read_to_end(&mut contents_ttc).unwrap();
            let _ = std::fs::write(cp_full_path.to_owned() + "/TTTCOF00.IMG", &contents_ttc); 
    }


pub fn ftp_download(host_port: &str, user: &str, password: &str, src: &str, cp_full_path: &str){
        info!("Trying to connect {}", &host_port);
    let ftp_timeout = Duration::seconds(5).to_std().unwrap();
    let socket_adr: SocketAddr = host_port.parse().expect("Unable to parse socket address");
    let mut ftp_stream = FtpStream::connect_timeout(socket_adr, ftp_timeout).expect("Couldn't connect to the server...");
    
    info!("Connected OK {}", &host_port);
    let _ = ftp_stream.login(user, password).unwrap();
    let _ = ftp_stream.cwd(src).unwrap();
    
    let remote_file_tts = ftp_stream.retr_as_buffer("TTSCOF00.IMG").unwrap();
    info!("Download TTSCOF00.IMG from {}", &host_port);  
    let mut file_tts = File::create(cp_full_path.to_owned() + "/TTSCOF00.IMG").unwrap();
            file_tts.write_all(&remote_file_tts.into_inner()).unwrap();
    let remote_file_ttc = ftp_stream.retr_as_buffer("TTTCOF00.IMG").unwrap();
    info!("Download TTTCOF00.IMG from {}", &host_port);
    let mut file_ttc = File::create(cp_full_path.to_owned() + "/TTTCOF00.IMG").unwrap();
    file_ttc.write_all(&remote_file_ttc.into_inner()).unwrap();
    let _ = ftp_stream.quit();
}
