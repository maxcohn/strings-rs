use std::io::Read;
use std::fs::File;
use std::env;


const BUFFER_SIZE: usize = 1024 * 1024;

fn is_printable(c: u8) -> bool {
    (c >= 0x20 && c <= 0x7e) || c == 0x09
}

fn main() {
    let min_len = 4;
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).expect("Couldn't open the file.");

    let mut cur_str: [u8; 256] = [0; 256];
    let mut str_counter = 0;

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let mut bytes_read = file.read(&mut buffer).unwrap();

    let mut print_str = String::with_capacity(20);

    while bytes_read > 0 {

        for b in buffer.iter() {
            //println!("{}", str_counter);
            if is_printable(*b) {
                cur_str[str_counter] = *b;
                str_counter += 1;
                
            } else {
                if str_counter >= min_len {
                    for i in 0..str_counter {
                        print_str.push(cur_str[i] as char);
                    }
                    println!("{}", print_str);
                    print_str.clear();
                }
                str_counter = 0;

            }
            
        }
        // refill the buffer with next bytes
        bytes_read = file.read(&mut buffer).unwrap();
    }

}
