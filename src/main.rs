extern crate ftp;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::io::{Read, Result};
use ftp::FtpStream;


fn read_as_bin2hex(handle: &mut impl Read, handle_tts: &mut impl Read, _fpath: &String) -> Result<()> {
    



    const READ_MAX_LEN: usize = 7;
    const READ_MAX_LEN_TTS: usize = 9;

    let mut bin = [0; READ_MAX_LEN];
    let mut bin_tts = [0; READ_MAX_LEN_TTS];
    
    let first_arg = env::args().skip(1).next();
    let fallback = "".to_owned();
    let _print = first_arg.unwrap_or(fallback);
    
    let mut _counter = 0;
    
    let _write_tts_print_img = _fpath.clone() + "/TTSCOF00.IMG";
    let _write_tts_print_txt = _fpath.clone() + "/TTSCOF00.txt";

    let _write_ttc_print_img = _fpath.clone() + "/TTTCOF00.IMG";
    let _write_ttc_print_txt = _fpath.clone() + "/TTTCOF00.txt";


    let rem_file_ttc = std::path::Path::new(&_write_ttc_print_txt).exists();
    let rem_file_tts = std::path::Path::new(&_write_tts_print_txt).exists();
    
    if rem_file_ttc{
        let _write_ttc_print_txt_rm = _write_ttc_print_txt.clone();
        fs::remove_file(_write_ttc_print_txt_rm).expect("Unable file for write");
    }

   if rem_file_tts{
        let _write_tts_print_txt_rm = _write_tts_print_txt.clone();
        fs::remove_file(_write_tts_print_txt_rm).expect("Unable file for write");
    }


    let mut file_ttc = OpenOptions::new().create_new(true).write(true).append(true).open(_write_ttc_print_txt).unwrap();
    let mut file_tts = OpenOptions::new().create_new(true).write(true).append(true).open(_write_tts_print_txt).unwrap();
    

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
        let _sec = &_hex[0..2];
        let _min = &_hex[2..4];
        let _hours = &_hex[4..6];
        let _day = &_hex[6..8];
        let _month = &_hex[8..10];
        let _year_1 = &_hex[12..14];
        let _year_2 = &_hex[10..12];
        
        let _file_state_tts = &_hex_tts[0..2];
        let _sec_tts = &_hex_tts[2..4];
        let _min_tts = &_hex_tts[4..6];
        let _hours_tts = &_hex_tts[6..8];
        let _day_tts = &_hex_tts[8..10];
        let _month_tts = &_hex_tts[10..12];
        let _year_1_tts = &_hex_tts[14..16];
        let _year_2_tts = &_hex_tts[12..14];
        let _storing_status_tts = &_hex_tts[16..18];
        
        let _file_state_encr = String::from(" ");
         match _file_state_tts.as_str(){
            "00"=>"OPEN",
            "01"=>"FULL",
            "02"=>"TRANSFERED",
            "03"=>"WAITING",
            "04"=>"COMPRESSING",
            "05"=>"UNUSEABLE",
            _ => "UNKNOWN",
        };
          
         println!("CF{}{}.DAT {}:{}:{} {}.{}.{}{} <-TTC | TTS-> {} {}:{}:{} {}.{}.{}{} {}",
                  _zero,_counter,_hours,_min,_sec,_day,_month,_year_1,_year_2,_file_state_tts,_hours_tts,_min_tts,_sec_tts,_day_tts,_month_tts,_year_1_tts,_year_2_tts,_storing_status_tts);
    
    if let Err(e) = writeln!(&mut file_ttc, "CF{}{}.DAT {}:{}:{} {}.{}.{}{}",_zero,_counter,_hours,_min,_sec,_day,_month,_year_1,_year_2) {
        eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(e) = writeln!(&mut file_tts, "CF{}{}.DAT {}:{}:{} {}.{}.{}{}",_zero,_counter,_hours,_min,_sec,_day,_month,_year_1,_year_2) {
        eprintln!("Couldn't write to file: {}", e);
    }

    



    }
    else if _print == "hex"{
           println!("CF{}{}.DAT {} <-TTC | TTS-> {}",_zero, _counter, _hex, _hex_tts);
    }
    else{
        println!("parameters not specified");
    }
     
    _counter += 1;   
    }  
    Ok(())
}

fn main() {
    let three_arg = env::args().skip(2).next();
    let fallback_3 = "".to_owned();
    let _ip = three_arg.unwrap_or(fallback_3);
   
    let _base_dir = String::from("/tmp/nsn/");
    let _result_dir = String::from("/result");
    let _full_path = _base_dir + &_ip + &_result_dir;
    let _cp_full_path = _full_path.clone();
    let _ip_port = _ip + ":21";

    fs::create_dir_all(_full_path).expect("Unable create directory");

    //get ftp result files
    let mut ftp_stream = FtpStream::connect(_ip_port).unwrap();
    let _ = ftp_stream.login("", "").unwrap();
    let _ = ftp_stream.cwd("ftpdir").unwrap();
    
    let remote_file_tts = ftp_stream.simple_retr("TTSCOF00.IMG").unwrap();
    let mut file_tts = File::create(_cp_full_path.to_owned() + "/TTSCOF00.IMG").unwrap();
    file_tts.write_all(&remote_file_tts.into_inner()).unwrap();
    
    let remote_file_ttc = ftp_stream.simple_retr("TTTCOF00.IMG").unwrap();
    let mut file_ttc = File::create(_cp_full_path.to_owned() + "/TTTCOF00.IMG").unwrap();
    file_ttc.write_all(&remote_file_ttc.into_inner()).unwrap();
    let _ = ftp_stream.quit();

    
    let _read_file = _cp_full_path.to_owned() + "/TTTCOF00.IMG";
    let _read_file_tts = _cp_full_path.to_owned() + "/TTSCOF00.IMG";
    

    let mut file = std::fs::File::open(_read_file).expect("Unable to open file");
    let mut file_s = std::fs::File::open(_read_file_tts).expect("Unable to open file");

    let _result = read_as_bin2hex(&mut file, &mut file_s, &_cp_full_path);
}













#[test]
fn test_with_file() {
    let mut file = std::fs::File::open("./TTTCOF00.IMG")
                                 .expect("Unable to open file");
    let _result = read_as_bin2hex(&mut file);
    assert!(_result.is_ok())
}
