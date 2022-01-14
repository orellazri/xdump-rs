use std::{env, fs::File, io::Read, process};

const OFFSET_STEP: usize = 16;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: xdump-rs <input file>");
        process::exit(1);
    }

    let input_filename = args.nth(1).unwrap();
    let mut file = File::open(&input_filename).unwrap();
    let metadata = file.metadata().unwrap();
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer).unwrap();

    let offset_padding = buffer.len().to_string().len();

    for offset in (0..buffer.len()).step_by(OFFSET_STEP) {
        // Print offset
        print!("{:0width$x}:    ", offset, width = offset_padding);

        let mut end = offset + OFFSET_STEP;
        let mut is_last_line = false;
        if end > buffer.len() {
            end = buffer.len();
            is_last_line = true;
        }

        // Print bytes
        buffer[offset..end].iter().for_each(|byte| print!("{:02x} ", byte));

        // Fill spaces for last line
        if is_last_line {
            for _ in 0..offset + OFFSET_STEP - end {
                print!("   ");
            }
        }

        // Print ASCII representation
        print!("    |");
        buffer[offset..end].iter().for_each(|byte| {
            if byte.is_ascii_graphic() {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        });

        print!("|");
        println!();
    }
}
