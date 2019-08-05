use std::fmt;

enum VMBytecode {
    IADD = 1, // int add
    ISUB = 2, // int sub
    IMUL = 3, // int multiplication
    ILT  = 4, // int less than
    IEQ  = 5, // int equal
    BR   = 6, // branch
    BRT  = 7, // branch if true
    BRF  = 8, // branch if false
    ICONST = 9,  // push constant int
    LOAD   = 10, // load from local context
    GLOAD  = 11, // load from global context
    STORE  = 12, // store in local context
    GSTORE = 13, // store in global memory
    PRINT  = 14, // print stack top
    POP    = 15, // throw away top of stack
    CALL   = 16, 
    RET    = 17, // return with/without value
    HALT   = 18,
}

impl From<i32> for VMBytecode {
    fn from(bytecode: i32) -> Self {
        match bytecode {
            1 => VMBytecode::IADD,
            2 => VMBytecode::ISUB,
            3 => VMBytecode::IMUL,
            4 => VMBytecode::ILT,
            5 => VMBytecode::IEQ,
            6 => VMBytecode::BR,
            7 => VMBytecode::BRT,
            8 => VMBytecode::BRF,
            9 => VMBytecode::ICONST,
            10 => VMBytecode::LOAD,
            11 => VMBytecode::GLOAD,
            12 => VMBytecode::STORE,
            13 => VMBytecode::GSTORE,
            14 => VMBytecode::PRINT,
            15 => VMBytecode::POP,
            16 => VMBytecode::CALL,
            17 => VMBytecode::RET,
            18 => VMBytecode::HALT,
            _ => panic!()
        }
    }
}

impl fmt::Display for VMBytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VMBytecode::IADD => write!(f, "IADD"),
            VMBytecode::ISUB => write!(f, "ISUB"),
            VMBytecode::IMUL => write!(f, "IMUL"),
            VMBytecode::ILT => write!(f, "ILT"),
            VMBytecode::IEQ => write!(f, "IEQ"),
            VMBytecode::BR => write!(f, "BR"),
            VMBytecode::BRT => write!(f, "BRT"),
            VMBytecode::BRF => write!(f, "BRF"),
            VMBytecode::ICONST => write!(f, "ICONST"),
            VMBytecode::LOAD => write!(f, "LOAD"),
            VMBytecode::GLOAD => write!(f, "GLOAD"),
            VMBytecode::STORE => write!(f, "STORE"),
            VMBytecode::GSTORE => write!(f, "GSTORE"),
            VMBytecode::PRINT => write!(f, "PRINT"),
            VMBytecode::POP => write!(f, "POP"),
            VMBytecode::CALL => write!(f, "CALL"),
            VMBytecode::RET => write!(f, "RET"),
            VMBytecode::HALT => write!(f, "HALT"),
        }
    }
}

struct VMInstruction<'a> {
    name: &'a str,
    nargs: i32,

}

impl<'a> VMInstruction<'a> {
    fn new(name: &'a str, nargs: i32) -> Self {
        VMInstruction {name: name, nargs: nargs}
    }
}

static VM_INSTRUCTIONS: [VMInstruction; 19] = [
    VMInstruction {name: "", nargs: 0},
    VMInstruction {name: "IADD", nargs: 0},
    VMInstruction {name: "ISUB", nargs: 0},
    VMInstruction {name: "IMUL", nargs: 0},
    VMInstruction {name: "ILT", nargs: 0},
    VMInstruction {name: "IEQ", nargs: 0},
    VMInstruction {name: "BR", nargs: 1},
    VMInstruction {name: "BRT", nargs: 1},
    VMInstruction {name: "BRF", nargs: 1},
    VMInstruction {name: "ICONST", nargs: 1},
    VMInstruction {name: "LOAD", nargs: 1},
    VMInstruction {name: "GLOAD", nargs: 1},
    VMInstruction {name: "STORE", nargs: 1},
    VMInstruction {name: "GSTORE", nargs: 1},
    VMInstruction {name: "PRINT", nargs: 0},
    VMInstruction {name: "POP", nargs: 0},
    VMInstruction {name: "CALL", nargs: 0},
    VMInstruction {name: "RET", nargs: 0},
    VMInstruction {name: "HALT", nargs: 0},
];

fn vm_print_instr<'a>(code: &'a Vec<i32>, ip: &usize) {
    let op = code[*ip];
    let instr: &VMInstruction = &VM_INSTRUCTIONS[op as usize];
    println!("{}: {}", ip, instr.name);
    if instr.nargs == 1 {
        println!(" {}", code[ip + 1]);
    } else if instr.nargs == 2 {
        println!(" {}, {}", code[ip + 1], code[ip + 2]);
    }
}

fn vm_print_stack<'a>(stack: &'a [i32], count: usize) {
    unimplemented!();
}

fn vm_print_globals<'a>(globals: &'a Vec<i32>, count: usize) {
    unimplemented!();
}



fn vm_exec(code: Vec<i32>, main: usize, size: usize, trace: bool) {
    let code: Vec<i32> = code;
    let mut stack: [i32; 100] = [0; 100];
    let mut globals: Vec<i32>; // represents global storage

    let mut ip: usize = main; // instruction pointer register
    let mut fp: usize;        // frame pointer register
    let mut sp: i32 = -1;   // stack pointer register
   
    let mut v: i32;

    while ip < code.len() {
        let opcode: VMBytecode = VMBytecode::from(code[ip]); // fetch
        let op: i32 = code[ip];
        if trace {
            vm_print_instr(&code, &ip);
        }
        ip = ip + 1;
        match opcode {
            VMBytecode::ICONST => {
                v = code[ip];
                ip = ip + 1;
                sp = sp + 1;
                stack[sp as usize] = v;
            },
            VMBytecode::PRINT => {
                v = stack[sp as usize];
                sp = sp - 1;
                println!("{}", v);
            },
            VMBytecode::GLOAD => {
            
            },
            VMBytecode::GSTORE => {
            
            },
            VMBytecode::HALT => return,
            _ => return
             
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hello_exec() {
        let hello = vec![
            VMBytecode::ICONST as i32, 1234, 
            VMBytecode::PRINT as i32,
            VMBytecode::HALT as i32
        ];
        vm_exec(hello, 0, 0, false);
    }

    #[test]
    fn check_hello_exec_tracing() {
        let hello = vec![
            VMBytecode::ICONST as i32, 1234, 
            VMBytecode::PRINT as i32,
            VMBytecode::HALT as i32
        ];
        vm_exec(hello, 0, 0, true);
    }

    #[test]
    fn check_global_storage() {
        let global_storage = vec![
            VMBytecode::ICONST as i32, 99,
            VMBytecode::GSTORE as i32, 0,
            VMBytecode::GLOAD as i32, 0,
            VMBytecode::PRINT as i32,
            VMBytecode::HALT as i32
        ];

        let datasize = 1;
        let mainip = 0;
        vm_exec(global_storage, mainip, datasize, true);
    }

}

