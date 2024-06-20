//example run: cargo run -- print sftp 127.0.0.1 22 user pass /home/dima/dev/Rust/nsn/example/ -1
use colored::Colorize;
use chrono::{NaiveDateTime, Duration, Local};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::io::{Read, Result};
use log::info;
use log4rs;
use nsn::ftp_download;
use nsn::sftp_download;




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
fn read_as_bin2hex(handle: &mut impl Read, handle_tts: &mut impl Read, _fpath: &String , num_edit_str: &String) -> Result<()> {

    let num_edit: i16 = num_edit_str.parse().expect("Block number not integer!");
        
    //размер байт для TTC
    const READ_MAX_LEN: usize = 7;
    //размер байт для TTS
    const READ_MAX_LEN_TTS: usize = 9;
    
    //иницилизируем статические массивы
    let mut bin = [0; READ_MAX_LEN];
    let mut bin_tts = [0; READ_MAX_LEN_TTS];
    
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
        let mut _zero = String::from("000");      
        
        if lenght == 2{
            _zero = String::from("00");
        }
        else if lenght == 3{
            _zero = String::from("0");
        }
        else if lenght == 4{
            _zero = String::from("");
        }
    
    
    if _print == "print"{
        let _sec_ttc = &_hex[0..2];
        let _min_ttc = &_hex[2..4];
        let _hours_ttc = &_hex[4..6];
        let _day_ttc = &_hex[6..8];
        let _month_ttc = &_hex[8..10];
        let _year_1_ttc = &_hex[12..14];
        let _year_2_ttc = &_hex[10..12];
        let _datetime_ttc = [&_hex[0..2], &_hex[2..4], &_hex[4..6], &_hex[6..8], &_hex[8..10], &_hex[10..12], &_hex[12..14]].concat();
        
        let _file_state_tts = &_hex_tts[0..2];
        let _sec_tts = &_hex_tts[2..4];
        let _min_tts = &_hex_tts[4..6];
        let _hours_tts = &_hex_tts[6..8];
        let _day_tts = &_hex_tts[8..10];
        let _month_tts = &_hex_tts[10..12];
        let _year_1_tts = &_hex_tts[14..16];
        let _year_2_tts = &_hex_tts[12..14];
        let _storing_status_tts = &_hex_tts[16..18];
        let _datetime_tts = [&_hex_tts[14..16], &_hex_tts[12..14], &_hex_tts[10..12], &_hex_tts[8..10], &_hex_tts[6..8], &_hex_tts[4..6], &_hex_tts[2..4]].concat();
        
        //расшифровка статусов
        let _file_state_encr = match _file_state_tts{
            "00" => "OPEN       ".red(),
            "01" => "FULL       ".green(),
            "02" => "TRANSFERED ".blue(),
            "03" => "WAITING    ".yellow(),
            "04" => "COMPRESSING".white(),
            "05" => "UNUSEABLE  ".black(),
             _   => "UNKNOWN    ".black(),
        };
    //если статус FULL
    if _file_state_tts == "01"{            
            let naive_datetime = NaiveDateTime::parse_from_str(&_datetime_tts, "%Y%m%d%H%M%S").unwrap();
            let add_naive_datetime = naive_datetime + Duration::seconds(60);
            let _string_datetime_tts = add_naive_datetime.to_string();
            let _new_datetime_ttc = [&_string_datetime_tts[17..19], &_string_datetime_tts[14..16], &_string_datetime_tts[11..13], &_string_datetime_tts[8..10], &_string_datetime_tts[5..7], &_string_datetime_tts[2..4], &_string_datetime_tts[0..2]].concat();
    //печать на экран в соответствии TTC-TTS записей
        println!("CF{}{}.DAT {}{}-{}-{} {}:{}:{} <-TTC | TTS-> {} {} {}{}-{}-{} {}:{}:{} {}",
                  _zero,_counter,_year_1_ttc,_year_2_ttc,_day_ttc,_month_ttc,_hours_ttc,_min_ttc,_sec_ttc, _file_state_tts,_file_state_encr,_year_1_tts,_year_2_tts,_day_tts,_month_tts,_hours_tts,_min_tts,_sec_tts,_storing_status_tts);

    //записываем в новый TTC-файл измененную запись с новым временем
       info!("CF{}{}.DAT: old time from {} is {}, new time for {} is {} in Nokia format: {}",  _zero,_counter, &_datetime_tts, _write_tts_print_img, _write_ttc_print_img, add_naive_datetime, _new_datetime_ttc);     
       let new_time_ttc_dec = hex::decode(_new_datetime_ttc).expect("Decoding failed new record");    
       let _ = file_new_ttc.write(&new_time_ttc_dec);  
         
     }
    else{
        println!("CF{}{}.DAT {}{}-{}-{} {}:{}:{} <-TTC | TTS-> {} {} {}{}-{}-{} {}:{}:{} {}",
         _zero,_counter,_year_1_ttc,_year_2_ttc,_day_ttc,_month_ttc,_hours_ttc,_min_ttc,_sec_ttc, _file_state_tts,_file_state_encr,_year_1_tts,_year_2_tts,_day_tts,_month_tts,_hours_tts,_min_tts,_sec_tts,_storing_status_tts);
    //записываем в новый TTC-файл не измененные записи
      let src_time_ttc_dec = hex::decode(_datetime_ttc).expect("Decoding failed old record");    
      let _ = file_new_ttc.write(&src_time_ttc_dec);
    }

    //блок обработки времени в TTC-файле вручную
    if num_edit == _counter && num_edit != -1{
    
        let ttc_manual_time = Local::now() + Duration::seconds(60);
        let ttc_manual_time_str = ttc_manual_time.to_string();
        let ttc_manual_nsn = [&ttc_manual_time_str[8..10], &ttc_manual_time_str[0..2] , &ttc_manual_time_str[2..4]].concat();
       println!("{} {} {:?} ---------", num_edit, ttc_manual_time_str, ttc_manual_nsn);
    }
    
    
    //записываем декодированный TTС в текстовый файл
    if let Err(e) = writeln!(&mut file_ttc, "CF{}{}.DAT {}:{}:{} {}.{}.{}{}",_zero,_counter,_hours_ttc,_min_ttc,_sec_ttc,_day_ttc,_month_ttc,_year_1_ttc,_year_2_ttc){
        eprintln!("Couldn't write to file: {}", e);
    }
    //убрали не ascii символы перед записью в текстовый файл TTS
    let _file_state_encr = _file_state_encr.replace(|c: char| !c.is_ascii(), "");
    
    //записываем декодированный TTS в текстовый файл
    if let Err(e) = writeln!(&mut file_tts, "CF{}{}.DAT {} {} {}:{}:{} {}.{}.{}{} {}",_zero,_counter,_file_state_tts,_file_state_encr,_hours_tts,_min_tts,_sec_tts,_day_tts,_month_tts,_year_1_tts,_year_2_tts,_storing_status_tts) {
        eprintln!("Couldn't write to file: {}", e);
    }

    }
    //если стоит ключ hex в командной строке
    else if _print == "hex"{                    
         println!("CF{}{}.DAT {} <-TTC | TTS-> {}",_zero, _counter, _hex, _hex_tts);
    }
    else if _print == "bin"{
        let binary_value_ttc = convert_to_binary_from_hex(&_hex);
        let binary_value_tts = convert_to_binary_from_hex(&_hex_tts);
         println!("CF{}{}.DAT {} <-TTC | TTS-> {}",_zero, _counter, binary_value_ttc, binary_value_tts);
    }

    else{
         println!("parameters not specified");
    }
     
    _counter += 1;   
    }  
    Ok(())
}

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    let fallback = "".to_owned();
    let three_args = env::args().skip(3).next();
    let _ip = three_args.unwrap_or(fallback.clone());    

    let four_args = env::args().skip(4).next();
    let host_port = four_args.unwrap_or(fallback.clone());
    
    let five_args = env::args().skip(5).next();
    let _user = five_args.unwrap_or(fallback.clone());

    let sixth_args = env::args().skip(6).next();
    let _pass = sixth_args.unwrap_or(fallback.clone());

    let seven_args = env::args().skip(7).next();
    let _srcdir = seven_args.unwrap_or(fallback.clone());

    let eight_args = env::args().skip(8).next();
    let _edit_block = eight_args.unwrap_or(fallback.clone());
   
    let base_dir = String::from("/tmp/nsn/");
    let result_dir = String::from("/result");
    let full_path = base_dir + &_ip + &result_dir;
    let cp_full_path = full_path.clone();

    fs::create_dir_all(full_path).expect("Unable create directory");

    let two_args = env::args().skip(2).next();
    let protocol = two_args.unwrap_or(fallback.clone());

    let ip_port = _ip + ":" + &host_port;

    //get ftp result files
 if protocol == "ftp"{
        ftp_download(&ip_port, &_user, &_pass, &_srcdir, &cp_full_path);
    }
    else{
        sftp_download(&ip_port, &_user, &_pass, &_srcdir, &cp_full_path);
    }


    
    let _read_file = cp_full_path.to_owned() + "/TTTCOF00.IMG";
    let _read_file_tts = cp_full_path.to_owned() + "/TTSCOF00.IMG";
    
    info!("Read file {}", _read_file);
    let mut file = std::fs::File::open(_read_file).expect("Unable to open file");
    info!("Read file {}", _read_file_tts);
    let mut file_s = std::fs::File::open(_read_file_tts).expect("Unable to open file");

    let _result = read_as_bin2hex(&mut file, &mut file_s, &cp_full_path, &_edit_block);
}




#[test]
fn test_with_file() {
    ///тестовая функция (читает только файл TTTCOF00.IMG в директории /tmp/)
   ///пример cargo test -- --nocapture
fn test_bin2hex(handle: &mut impl Read) -> Result<()> {
    const READ_MAX_LEN: usize = 7;
    let mut bin = [0; READ_MAX_LEN];
    loop {
        let bytes_read = handle.take(READ_MAX_LEN as u64).read(&mut bin)?;
        
        if bytes_read == 0 { break; } // EOF

        let hex = bin[..bytes_read].iter().map(|byte|format!("{byte:02x?}")).collect::<String>();
        println!("{hex}");
    }
    Ok(())
} 
   
    let mut file = std::fs::File::open("/tmp/TTTCOF00.IMG").expect("Unable to open file");
    let _result = test_bin2hex(&mut file);

    assert!(_result.is_ok())
}
