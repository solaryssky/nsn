use std::net::TcpStream;
use std::io::{Read, Write, Result};
use std::fs::{File, OpenOptions};
use ssh2::Session;
use std::path::Path;
use std::net::SocketAddr;
use suppaftp::FtpStream;
use log::{info, error};
use uuid::Uuid;
use colored::Colorize;
use chrono::{NaiveDateTime, Duration, Local};
use std::env;
use std::fs;
use postgres::{Client, NoTls};



pub fn sftp_download(host_port: &str, user: &str, password: &str, src: &str, cp_full_path: &str, my_uuid: &Uuid, spanid: &str){
    info!("{}: trying to connect {} {}", &host_port, &my_uuid, &spanid);
   
    match TcpStream::connect(host_port){
           Ok(tcp) => {
                info!("{}: connect succeful {} {}", &host_port, &my_uuid, &spanid);
                let mut sess = Session::new().unwrap();
                sess.set_tcp_stream(tcp);
                sess.set_compress(true);
                sess.timeout();
                sess.set_timeout(5000);
                sess.handshake().unwrap();

               match sess.userauth_password(user, password){
                        Ok(()) => info!("{}: auth OK from user: {} {} {}",&host_port, &user, &my_uuid, &spanid),
                        Err(e) => error!("{}: connect error by sftp protocol description: {} {} {}", &host_port, e, &my_uuid, &spanid)
                    };

                let mut contents_tts:Vec<u8> = Vec::new();
                let mut contents_ttc:Vec<u8> = Vec::new();
                let sftp = sess.sftp().unwrap();
                 
                info!("{}: download TTTSOF00.IMG {} {}", &host_port, &my_uuid, &spanid);
                let mut stream_tts = sftp.open(Path::new(&(src.to_owned() + "/TTSCOF00.IMG"))).map_err(|_| error!("{}: download ERROR TTTSOF00.IMG {} {}", &host_port, &my_uuid, &spanid)).unwrap();
                        stream_tts.read_to_end(&mut contents_tts).unwrap();
                let _ = std::fs::write(cp_full_path.to_owned() + "/TTSCOF00.IMG", &contents_tts);

                let mut stream_ttc = sftp.open(Path::new(&(src.to_owned() + "/TTTCOF00.IMG"))).map_err(|_| error!("{}: download ERROR TTTCOF00.IMG {} {}", &host_port, &my_uuid, &spanid)).unwrap();
                info!("{}: download TTTCOF00.IMG {} {}", &host_port, &my_uuid, &spanid);
                        stream_ttc.read_to_end(&mut contents_ttc).unwrap();
                let _ = std::fs::write(cp_full_path.to_owned() + "/TTTCOF00.IMG", &contents_ttc);
                info!("{}: quit {} {}", &host_port, &my_uuid, &spanid); 
                

    },
    Err(e) => {
        error!("{}: connect error by sftp protocol description: {} {} {}", &host_port, e, &my_uuid, &spanid);
        std::process::exit(1);
    }
}
    
    }


pub fn ftp_download(host_port: &str, user: &str, password: &str, src: &str, cp_full_path: &str, my_uuid: &Uuid, spanid: &String){
        info!("{}: trying to connect {} {}", &host_port, &my_uuid, &spanid);
    let ftp_timeout = Duration::seconds(5).to_std().unwrap();
    let socket_adr: SocketAddr = host_port.parse().expect("Unable to parse socket address");
    
    //let mut ftp_stream = FtpStream::connect_timeout(socket_adr, ftp_timeout).expect("Couldn't connect to the server...");

    match FtpStream::connect_timeout(socket_adr, ftp_timeout){
        Ok(mut ftp_stream) => {
                info!("{}: connect succeful {} {}", &host_port, &my_uuid, &spanid);
                //let _ = ftp_stream.login(user, password).unwrap();
                
                match ftp_stream.login(user, password){
                        Ok(()) => info!("{}: auth OK from user: {} {} {}",&host_port, &user, &my_uuid, &spanid),
                        Err(e) => error!("{}: connect error by sftp protocol description: {} {} {}", &host_port, e, &my_uuid, &spanid)
                    };

                let _ = ftp_stream.cwd(src).unwrap();
                
                let remote_file_tts = ftp_stream.retr_as_buffer("TTSCOF00.IMG").unwrap();
                info!("{}: download TTSCOF00.IMG {} {}", &host_port, &my_uuid, &spanid); 
                info!("{}: copy TTSCOF00.IMG to local file {} {} {}",&host_port, cp_full_path, &my_uuid, &spanid); 
                let mut file_tts = File::create(cp_full_path.to_owned() + "/TTSCOF00.IMG").unwrap();
                        file_tts.write_all(&remote_file_tts.into_inner()).unwrap();
                info!("{}: download TTTCOF00.IMG {} {}", &host_port, &my_uuid, &spanid);
                let remote_file_ttc = ftp_stream.retr_as_buffer("TTTCOF00.IMG").unwrap();
                info!("{}: copy TTTCOF00.IMG to local file {} {} {}",&host_port, cp_full_path, &my_uuid, &spanid);
                let mut file_ttc = File::create(cp_full_path.to_owned() + "/TTTCOF00.IMG").unwrap();
                        file_ttc.write_all(&remote_file_ttc.into_inner()).unwrap();
                info!("{}: quit {} {}", &host_port, &my_uuid, &spanid);
                let _ = ftp_stream.quit();
        },
        Err(e) => {
                error!("{}: connect error by ftp protocol description: {} {} {}", &host_port, e, &my_uuid, &spanid);
                std::process::exit(1);
            } 
    }

}


//перевод в двоичный
fn convert_to_binary_from_hex(hex: &str) -> String {
    hex[0..].chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}


//рабочая функция
pub fn read_as_bin2hex(ip: &str, handle: &mut impl Read, handle_tts: &mut impl Read, _fpath: &String, num_edit_str: &String, sentry_event_id: &Uuid, spanid: &String) -> Result<()> {

    let num_edit: i16 = num_edit_str.parse().expect("Block number not integer!");
    let mut out_string= String::from("{\"key1\": [");

        
    //размер байт для TTC
    const READ_MAX_LEN: usize = 7;
    //размер байт для TTS
    const READ_MAX_LEN_TTS: usize = 9;
    
    //иницилизируем статические массивы
    let mut bin = [0; READ_MAX_LEN];
    let mut bin_tts = [0; READ_MAX_LEN_TTS];
    
    //print result args key
    let first_arg = env::args().skip(1).next();
    let fallback = "".to_owned();
    let _print = first_arg.unwrap_or(fallback);


    
    let mut _counter: i16 = 0;
    
    let _write_tts_print_img = _fpath.clone() + "/TTSCOF00.IMG";
    let _write_tts_print_txt = _fpath.clone() + "/TTSCOF00.txt";

    let _write_new_ttc = _fpath.clone() + "/new_TTTCOF00.IMG";

    let _write_ttc_print_img = _fpath.clone() + "/TTTCOF00.IMG";
    let _write_ttc_print_txt = _fpath.clone() + "/TTTCOF00.txt";


    let rem_file_ttc = std::path::Path::new(&_write_ttc_print_txt).exists();
    let rem_file_tts = std::path::Path::new(&_write_tts_print_txt).exists();
    let rem_file_new_ttc = std::path::Path::new(&_write_new_ttc).exists();
    
    if rem_file_ttc{
        let _write_ttc_print_txt_rm = _write_ttc_print_txt.clone();
        fs::remove_file(_write_ttc_print_txt_rm).expect("Unable delete file TTTCOF00.txt");
    }

   if rem_file_tts{
        let _write_tts_print_txt_rm = _write_tts_print_txt.clone();
        fs::remove_file(_write_tts_print_txt_rm).expect("Unable delete file TTSCOF00.txt");        
    }

    if rem_file_new_ttc{
        let _write_new_ttc_rm = _write_new_ttc.clone();
        fs::remove_file(_write_new_ttc_rm).expect("Unable delete file new_TTTCOF00.IMG");
    }


    let mut file_ttc = OpenOptions::new().create_new(true).write(true).append(true).open(_write_ttc_print_txt).unwrap();
    let mut file_tts = OpenOptions::new().create_new(true).write(true).append(true).open(_write_tts_print_txt).unwrap();
    let mut file_new_ttc = OpenOptions::new().create_new(true).write(true).append(true).open(_write_new_ttc).unwrap();
    
    

    loop {
        let bytes_read = handle.take(READ_MAX_LEN as u64).read(&mut bin)?;
        let bytes_read_tts = handle_tts.take(READ_MAX_LEN_TTS as u64).read(&mut bin_tts)?;

        if bytes_read == 0 { 
            break; 
        } // EOF

        let _hex = bin[..bytes_read].iter().map(|byte|format!("{byte:02x?}")).collect::<String>();        
        let _hex_tts = bin_tts[..bytes_read_tts].iter().map(|byte|format!("{byte:02x?}")).collect::<String>();                     

    
        let _str: String = _counter.to_string();
        let lenght = _str.len();    
        let mut zero = String::from("000");      
        
        if lenght == 2{
            zero = String::from("00");
        }
        else if lenght == 3{
            zero = String::from("0");
        }
        else if lenght == 4{
            zero = String::from("");
        }
    
    
    if _print == "print"{
        let sec_ttc = &_hex[0..2];
        let min_ttc = &_hex[2..4];
        let hours_ttc = &_hex[4..6];
        let day_ttc = &_hex[6..8];
        let month_ttc = &_hex[8..10];
        let year_1_ttc = &_hex[12..14];
        let year_2_ttc = &_hex[10..12];
        let datetime_ttc = [&_hex[0..2], &_hex[2..4], &_hex[4..6], &_hex[6..8], &_hex[8..10], &_hex[10..12], &_hex[12..14]].concat();
        
        let file_state_tts = &_hex_tts[0..2];
        let sec_tts = &_hex_tts[2..4];
        let min_tts = &_hex_tts[4..6];
        let hours_tts = &_hex_tts[6..8];
        let day_tts = &_hex_tts[8..10];
        let month_tts = &_hex_tts[10..12];
        let year_1_tts = &_hex_tts[14..16];
        let year_2_tts = &_hex_tts[12..14];
        let storing_status_tts = &_hex_tts[16..18];
        let datetime_tts = [&_hex_tts[14..16], &_hex_tts[12..14], &_hex_tts[10..12], &_hex_tts[8..10], &_hex_tts[6..8], &_hex_tts[4..6], &_hex_tts[2..4]].concat();
        
        //расшифровка статусов
        let file_state_encr = match file_state_tts{
            "00" => "OPEN       ".red(),
            "01" => "FULL       ".green(),
            "02" => "TRANSFERED ".blue(),
            "03" => "WAITING    ".yellow(),
            "04" => "COMPRESSING".cyan(),
            "05" => "UNUSEABLE  ".magenta(),
             _   => "UNKNOWN    ".white(),
        };

    let cf_filename = "CF".to_owned() + &zero + &_counter.to_string() + ".DAT";
    let datetime_str_ttc = year_1_ttc.to_owned() + year_2_ttc + "-" + day_ttc + "-" + month_ttc + " " + hours_ttc + ":" + min_ttc + ":" + sec_ttc;
    let datetime_str_tts = year_1_tts.to_owned() + year_2_tts + "-" + day_tts + "-" + month_tts + " " + hours_tts + ":" + min_tts + ":" + sec_tts;
    let naivedatetime_str_ttc = NaiveDateTime::parse_from_str(&datetime_str_ttc, "%Y-%d-%m %H:%M:%S").unwrap_or_default();
    let naivedatetime_str_tts = NaiveDateTime::parse_from_str(&datetime_str_tts, "%Y-%d-%m %H:%M:%S").unwrap_or_default();

    //если статус FULL
    if file_state_tts == "01"{            
            let naive_datetime = NaiveDateTime::parse_from_str(&datetime_tts, "%Y%m%d%H%M%S").unwrap();
            let add_naive_datetime = naive_datetime + Duration::seconds(60);
            let _string_datetime_tts = add_naive_datetime.to_string();
            let _new_datetime_ttc = [&_string_datetime_tts[17..19], &_string_datetime_tts[14..16], &_string_datetime_tts[11..13], &_string_datetime_tts[8..10], &_string_datetime_tts[5..7], &_string_datetime_tts[2..4], &_string_datetime_tts[0..2]].concat();
    //печать на экран в соответствии TTC-TTS записей
        println!("{} {} <-TTC | TTS-> {} {} {} {}",
                  &cf_filename, &datetime_str_ttc, file_state_tts, file_state_encr, &datetime_str_tts, storing_status_tts);

    //записываем в новый TTC-файл измененную запись с новым временем
       info!("{}: old time from {} is {}, new time for {} is {} in Nokia format: {} {} {}",  &cf_filename, &datetime_tts, _write_tts_print_img, _write_ttc_print_img, add_naive_datetime, _new_datetime_ttc, &sentry_event_id, &spanid);     
       let new_time_ttc_dec = hex::decode(_new_datetime_ttc).expect("Decoding failed new record");    
       let _ = file_new_ttc.write(&new_time_ttc_dec);  
         
     }
    else{
        println!("{} {} <-TTC | TTS-> {} {} {} {}",
                  &cf_filename, &datetime_str_ttc, file_state_tts, file_state_encr, &datetime_str_tts, storing_status_tts);
       
    //записываем в новый TTC-файл не измененные записи
      let src_time_ttc_dec = hex::decode(datetime_ttc).expect("Decoding failed old record");    
      let _ = file_new_ttc.write(&src_time_ttc_dec);
    }

    //пишем в базу статистики текущее состояние TTC-TTS записей
    let file_state_encr_cut_ansi:String = file_state_encr.chars().filter(|c|c.is_ascii()).collect();
   
    //формирруем json для его записи в базу статистики текущее состояние TTC-TTS записей
    out_string.push_str(&("{\"file\":\"".to_owned() + &cf_filename + "\",\"ttc_date\":\"" + &naivedatetime_str_ttc.to_string()+ "\",\"tts_status_code\":\"" + &file_state_tts + "\",\"tts_status_info\":\"" + &file_state_encr_cut_ansi.trim() + "\",\"tts_date\":\"" + &naivedatetime_str_tts.to_string() + "\",\"tts_storing_status\":\"" + &storing_status_tts + "\",\"uuid\":\"" + &sentry_event_id.to_string() + "\",\"ip_port\":\"" + &ip + "\"},"));
   
   
   
    // let _ =  conn.execute("INSERT INTO status (file, ttc_date, tts_status_code, tts_status_info, tts_date, tts_storing_status, uuid, ip_port) values ($1, $2, $3, $4, $5, $6, $7, $8)", &[&cf_filename, &naivedatetime_str_ttc, &file_state_tts, &file_state_encr_cut_ansi, &naivedatetime_str_tts, &storing_status_tts, &sentry_event_id, &ip],); 
    

    //блок обработки времени в TTC-файле вручную
    if num_edit == _counter && num_edit != -1{
    
        let ttc_manual_time = Local::now() + Duration::seconds(60);
        let ttc_manual_time_str = ttc_manual_time.to_string();
        let ttc_manual_nsn = [&ttc_manual_time_str[8..10], &ttc_manual_time_str[0..2] , &ttc_manual_time_str[2..4]].concat();
       println!("{} {} {:?} ---------", num_edit, ttc_manual_time_str, ttc_manual_nsn);
    }
    
    
    //записываем декодированный TTС в текстовый файл
    if let Err(e) = writeln!(&mut file_ttc, "CF{}{}.DAT {}:{}:{} {}.{}.{}{}",zero,_counter,hours_ttc,min_ttc,sec_ttc,day_ttc,month_ttc,year_1_ttc,year_2_ttc){
        eprintln!("Couldn't write to file: {}", e);
    }
    //убрали не ascii символы перед записью в текстовый файл TTS
    let file_state_encr = file_state_encr.replace(|c: char| !c.is_ascii(), "");
    
    //записываем декодированный TTS в текстовый файл
    if let Err(e) = writeln!(&mut file_tts, "CF{}{}.DAT {} {} {}:{}:{} {}.{}.{}{} {}",zero,_counter,file_state_tts,file_state_encr,hours_tts,min_tts,sec_tts,day_tts,month_tts,year_1_tts,year_2_tts,storing_status_tts) {
        eprintln!("Couldn't write to file: {}", e);
    }

    }
    //если стоит ключ hex в командной строке
    else if _print == "hex"{                    
         println!("CF{}{}.DAT {} <-TTC | TTS-> {}",zero, _counter, _hex, _hex_tts);
    }
    else if _print == "bin"{
        let binary_value_ttc = convert_to_binary_from_hex(&_hex);
        let binary_value_tts = convert_to_binary_from_hex(&_hex_tts);
         println!("CF{}{}.DAT {} <-TTC | TTS-> {}",zero, _counter, binary_value_ttc, binary_value_tts);
    }

    else{
         println!("parameters not specified");
    }
     
    _counter += 1;   
    }  

    out_string.truncate(out_string.len() - 1);
    out_string.push_str(&("]}"));
    let json_data: serde_json::Value = serde_json::from_str(&out_string).expect("Can't parse json");

     //соединение с базой статистики
    let client = Client::connect("postgresql://nsnuser:At5#lOq5wADvlh8t@11.60.4.230:5432/nsnlog", NoTls); 

    match client {
        Ok(mut client) => {
            let stmt = client.prepare("INSERT INTO status (json, host) VALUES ($1, $2)").unwrap();
                       client.execute(&stmt, &[&json_data, &ip]).expect("Failed to execute statement to ");
      
            info!("{}: json data inserted to database {} {}",&ip, &sentry_event_id, &spanid);
        }
        Err(err) => error!("{}: json data fail to database: {} {} {}", &ip, err, &sentry_event_id, &spanid),
    }



    Ok(())
}
