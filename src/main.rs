
use std::io::{Read, Result};
use std::env;

fn read_as_bin2hex(handle: &mut impl Read) -> Result<()> {
    const READ_MAX_LEN: usize = 7;
    let mut bin = [0; READ_MAX_LEN];
    
    let first_arg = env::args().skip(1).next();
    let fallback = "".to_owned();
    let _print = first_arg.unwrap_or(fallback);

    loop {
        let bytes_read = handle.take(READ_MAX_LEN as u64)
                               .read(&mut bin)?;

        if bytes_read == 0 { 
            break; 
        } // EOF

        let _hex = bin[..bytes_read].iter()
                        .map(|byte|format!("{byte:02x?}"))
                        .collect::<String>();                     

    
    if _print == "print"{
        let _sec = &_hex[0..2];
        let _min = &_hex[2..4];
        let _hours = &_hex[4..6];
        let _year_1 = &_hex[12..14];
        let _year_2 = &_hex[10..12];
        let _month = &_hex[8..10];
        let _day = &_hex[6..8];
        
           println!("{}:{}:{} {}.{}.{}{}", _hours,_min,_sec,_day,_month,_year_1,_year_2);
    }
    else if _print == "hex"{
           println!("{_hex}");
    }
    else{
        println!("parameters not specified");
    }
       
    }    
    Ok(())
}

fn main() {
    let two_arg = env::args().skip(2).next();
    let fallback_2 = "".to_owned();
    let _path = two_arg.unwrap_or(fallback_2);

    let mut file = std::fs::File::open(_path).expect("Unable to open file");
    let _result = read_as_bin2hex(&mut file);
}

#[test]
fn test_with_file() {
    let mut file = std::fs::File::open("./TTTCOF00.IMG")
                                 .expect("Unable to open file");
    let _result = read_as_bin2hex(&mut file);
    assert!(_result.is_ok())
}
