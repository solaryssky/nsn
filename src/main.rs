//build command: 
//example run: cargo run -- print sftp 127.0.0.1 22 'USER' 'PASS' /path/path-000 -1
//example run: cargo run -- print sftp localhost 22 'USER' 'PASS' /path/to/example/ -1
//nsn print sftp 127.0.0.1 22 'USER' 'PASS' /path/path-000 -1

use std::env;
use std::fs;
use log::info;
use log4rs;
use nsn::ftp_download;
use nsn::sftp_download;
use nsn::read_as_bin2hex;
use uuid::Uuid;
use whoami::fallible;


fn main() {

    let hostname = fallible::hostname().unwrap();

    let _guard = sentry::init(("https://url.ru/1596", 
    sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 0.2, //send 20% of transaction to sentry
            ..Default::default()
        }));

        sentry::configure_scope(|scope| {
            scope.set_user(Some(sentry::User {
                id: Some(hostname.clone()),
                email: Some("dima@yandex.ru".to_owned()),
                username: Some(whoami::username()),                
                ..Default::default()
            }));
           scope.set_tag("Nokia downloader", &hostname);
        });
    
        let tx_ctx = sentry::TransactionContext::new(&hostname,"main transaction",);
        let transaction = sentry::start_transaction(tx_ctx);
                    sentry::capture_message("Im start!", sentry::Level::Info);
                    

    //ip-host args key  
    let _ip = env::args().nth(3).unwrap_or_else(|| {
        eprintln!("Error: IP not set in arguments key");
        std::process::exit(1);
    }
    );
    let work_dir = String::from("/path/nsn");
    let full_path = work_dir.to_owned() + "/result/" + &_ip;
    let cp_full_path = full_path.clone();

    
    log4rs::init_file(work_dir + "/logging_config.yaml", Default::default()).expect("not found config file for log4rs");

    //let fallback = "".to_owned();


    //port args key
    //let four_args = env::args().skip(4).next();
    //let host_port = four_args.unwrap_or(fallback.clone());
    let host_port = env::args().nth(4).unwrap_or_else(|| {
        eprintln!("Error: Host and port not set in arguments key");
        std::process::exit(1);
    }
    );
    
    //user args key
    //let five_args = env::args().skip(5).next();
    //let _user = five_args.unwrap_or(fallback.clone());
    let _user = env::args().nth(5).unwrap_or_else(|| {
        eprintln!("Error: User login not set in arguments key");
        std::process::exit(1);
    }
    );
    

    //password args key
    //let sixth_args = env::args().skip(6).next();
    //let _pass = sixth_args.unwrap_or(fallback.clone());
    let _pass = env::args().nth(6).unwrap_or_else(|| {
        eprintln!("Error: User password not set in arguments key");
        std::process::exit(1);
    }
    );

    //source directory for file args key
    //let seven_args = env::args().skip(7).next();
    //let _srcdir = seven_args.unwrap_or(fallback.clone());
    let _srcdir = env::args().nth(7).unwrap_or_else(|| {
        eprintln!("Error: Source directory not set in arguments key");
        std::process::exit(1);
    }
    );



    //edit mode args key (edit block number or disable key "-1")
    //let eight_args = env::args().skip(8).next();
    //let _edit_block = eight_args.unwrap_or(fallback.clone());
    let _edit_block = env::args().nth(8).unwrap_or_else(|| {
        eprintln!("Error: Edit mode not set in arguments key (set edit block number or use disable key '-1')");
        std::process::exit(1);
    }
    );

    fs::create_dir_all(full_path).expect("Unable create directory");

    //type protocol args key (ftp/sftp)
   // let two_args = env::args().skip(2).next();
    //let protocol = two_args.unwrap_or(fallback.clone());

    let protocol = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Error: Not set protocol: ftp or sftp in key");
        std::process::exit(1);
    }
    );

    let ip_port = _ip + ":" + &host_port;
    let sentry_event_id:Uuid = sentry::last_event_id().expect("Cannot get event_id from Sentry");

    //get ftp result files
    let span_ftp = transaction.start_child("start ftp/sftp", &ip_port);
 if protocol == "ftp"{
        ftp_download(&ip_port, &_user, &_pass, &_srcdir, &cp_full_path, &sentry_event_id, &span_ftp.get_span_id().to_string());
    }
    else{
        sftp_download(&ip_port, &_user, &_pass, &_srcdir, &cp_full_path, &sentry_event_id, &span_ftp.get_span_id().to_string());
    }
    span_ftp.finish();

    let _read_file = cp_full_path.to_owned() + "/TTTCOF00.IMG";
    let _read_file_tts = cp_full_path.to_owned() + "/TTSCOF00.IMG";
    
    let span_read_ttc = transaction.start_child("read file", "TTTCOF00.IMG");
    info!("{}: read file {} {} {}",&ip_port, _read_file, &sentry_event_id, &span_read_ttc.get_span_id().to_string());
    let mut file = std::fs::File::open(_read_file).expect("Unable to open file");
    span_read_ttc.finish();

    let span_read_tts = transaction.start_child("read file", "TTSCOF00.IMG");
    info!("{}: read file {} {} {}",&ip_port, _read_file_tts, &sentry_event_id, &span_read_tts.get_span_id().to_string());
    let mut file_s = std::fs::File::open(_read_file_tts).expect("Unable to open file");
    span_read_tts.finish();

    let span_result = transaction.start_child("result", "execute read_as_bin2hex");
    let spanid_result = &span_result.get_span_id().to_string();
    let _result = read_as_bin2hex(&ip_port, &mut file, &mut file_s, &cp_full_path, &_edit_block, &sentry_event_id, &spanid_result);
    span_result.finish();
    info!("{}: end of work {}",&ip_port, &sentry_event_id);
    transaction.finish();
}





//UNIT TEST BLOCK

#[test]
///тестовая функция (читает только файл TTTCOF00.IMG в директории /tmp/)
///пример cargo test -- --nocapture

fn test_with_file() {
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

//test for convert_to_binary_from_hex и to_binary
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary('0'), "0000");
        assert_eq!(to_binary('1'), "0001");
        assert_eq!(to_binary('A'), "1010");
        assert_eq!(to_binary('F'), "1111");
        assert_eq!(to_binary('G'), ""); // Неверный символ
    }
 
    #[test]
    fn test_convert_to_binary_from_hex() {
        assert_eq!(convert_to_binary_from_hex("0"), "0000");
        assert_eq!(convert_to_binary_from_hex("1A"), "00011010");
        assert_eq!(convert_to_binary_from_hex("FF"), "11111111");
        assert_eq!(convert_to_binary_from_hex("1F3"), "000111110011");
    }
}

//тест для проверки записи и чтения файлов
 
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom};
    use tempfile::NamedTempFile;
 
    #[test]
    fn test_read_as_bin2hex_with_files() {
        // Создаем временные файлы для тестирования
        let mut ttc_file = NamedTempFile::new().unwrap();
        let mut tts_file = NamedTempFile::new().unwrap();
 
        // Записываем тестовые данные в файлы
        let ttc_data = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE];
        let tts_data = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99];
 
        ttc_file.write_all(&ttc_data).unwrap();
        tts_file.write_all(&tts_data).unwrap();
 
        // Перемещаем указатель в начало файлов
        ttc_file.seek(SeekFrom::Start(0)).unwrap();
        tts_file.seek(SeekFrom::Start(0)).unwrap();
 
        // Вызываем тестируемую функцию
        let result = read_as_bin2hex(
            "127.0.0.1:8080",
            &mut ttc_file,
            &mut tts_file,
            &"/tmp/nsn/127.0.0.1/result".to_string(),
            &"0".to_string(),
        );
 
        // Проверяем, что функция завершилась успешно
        assert!(result.is_ok());
    }
}

//тест с моком для базы данных
 
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
 
    mock! {
        pub DbClient {}
        impl Client for DbClient {
            fn execute(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, postgres::Error>;
        }
    }
 
    #[test]
    fn test_database_insert() {
        let mut mock_db = MockDbClient::new();
 
        // Настраиваем мок для ожидаемого вызова
        mock_db
            .expect_execute()
            .with(
                eq("INSERT INTO status (file, ttc_date, tts_status_code, tts_status_info, tts_date, tts_storing_status, uuid, ip_port) values ($1, $2, $3, $4, $5, $6, $7, $8)"),
                always(),
            )
            .times(1)
            .returning(|_, _| Ok(1));
 
        // Вызываем тестируемую функцию с моком базы данных
        let result = read_as_bin2hex(
            "127.0.0.1:8080",
            &mut Cursor::new(vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]),
            &mut Cursor::new(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99]),
            &"/tmp/nsn/127.0.0.1/result".to_string(),
            &"0".to_string(),
        );
 
        // Проверяем, что функция завершилась успешно
        assert!(result.is_ok());
    }
}
