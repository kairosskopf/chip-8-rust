fn main() {
    let rom_bytes = load_file();
    for i in (0..rom_bytes.len()).step_by(2) {
        let tmp = &rom_bytes[i..i + 2];
    }
}

fn load_file() -> Vec<u8> {
    use std::fs;

    fs::read("/home/kai/chip-8/roms/helloworld.rom")
        .expect("Should have been able to read the file")
}

fn byte_to_opcode(opt_bytes: &[u8]) {}

#[derive(Debug)]
enum Opts {
    CLS,
}
// 0NNN 	Call
// 00E0 	Display
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
