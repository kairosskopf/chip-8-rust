fn main() {
    let rom_bytes = load_file();
    for i in (0..rom_bytes.len()).step_by(2) {
        let tmp = &rom_bytes[i..i + 2];
        let opt = byte_to_opcode(tmp);
        if !matches!(opt, Opts::NotYetImpl) {
            println!("{:?}", opt);
        } else {
            print!("{:02X?}{:02X?}\n", tmp[0], tmp[1]);
        }
    }
}

fn load_file() -> Vec<u8> {
    use std::env;
    use std::fs;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    fs::read(file_path).expect("Should have been able to read the file")
}

fn byte_to_opcode(opt_bytes: &[u8]) -> Opts {
    match opt_bytes {
        [0x00, 0xE0] => Opts::CLS,
        [0x60..=0x6F, 0x00..=0xFF] => Opts::SetVxToByte {
            x_v: (opt_bytes[0] - 0x60),
            byte: opt_bytes[1],
        },
        [0xA0..=0xAF, 0x00..=0xFF] => {
            Opts::SetATo((((opt_bytes[0] - 0xA0) as u16) << 8) | (opt_bytes[1] as u16))
        }
        [0xF0..=0xFF, 0x0A] => Opts::WaitForKeyStoreInVx(opt_bytes[0] - 0xF0),
        [0xF0..=0xFF, 0x18] => Opts::SetSoundTimerToVx(opt_bytes[0] - 0xF0),
        [0xF0..=0xFF, 0x29] => Opts::SetIToLocationOfSpriteDigitVx(opt_bytes[0] - 0xF0),
        [0xD0..=0xDF, 0x00..=0xFF] => Opts::DisplayNByteSpriteFromIAtXY {
            x: (opt_bytes[0] - 0xD0),
            y: ((opt_bytes[1] & 0xF0) >> 4),
            n: (opt_bytes[1] & 0x0F),
        },
        [0x10..=0x1F, 0x00..=0xFF] => Opts::SetPC((((opt_bytes[0] - 0x10) as u16) << 8) | (opt_bytes[1] as u16)),
        _ => Opts::NotYetImpl,
    }
}

#[derive(Debug)]
enum Opts {
    CLS,
    SetVxToByte { x_v: u8, byte: u8 },
    SetATo(u16),
    WaitForKeyStoreInVx(u8),
    SetSoundTimerToVx(u8),
    SetIToLocationOfSpriteDigitVx(u8),
    DisplayNByteSpriteFromIAtXY { x: u8, y: u8, n: u8 },
    SetPC(u16),
    NotYetImpl,
}

// 0NNN 	Call
// 00EE 	Flow
// 1NNN 	Flow
// 2NNN 	Flow
// 3XNN 	Cond
// 4XNN 	Cond
// 5XY0 	Cond
// 6XNN 	Const
// 7XNN 	Const
// 8XY0 	Assig
// 8XY1 	BitOp
// 8XY2 	BitOp
// 8XY3	BitOp
// 8XY4 	Math
// 8XY5 	Math
// 8XY6 	BitOp
// 8XY7 	Math
// 8XYE 	BitOp
// 9XY0 	Cond
// ANNN 	MEM
// BNNN 	Flow
// CXNN 	Rand
// DXYN 	Display
// EX9E 	KeyOp
// EXA1 	KeyOp
// FX07 	Timer
// FX0A 	KeyOp
// FX15 	Timer
// FX18 	Sound
// FX1E 	MEM
// FX29 	MEM
// FX33 	BCD
// FX55 	MEM
// FX65 	MEM
// }
