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

struct VMInstruction {
    name: [String;8],
    nargs: i32,

}

impl VMInstruction {
    fn new(&self, name: [String;8], nargs: i32) -> Self {
        VMInstruction {name: name, nargs: nargs}
    }
}

fn vm_exec(code: Vec<i32>, main: usize, size: usize) {
    let code: Vec<i32> = code;
    let mut stack: [i32; 100] = [0; 100];
    let mut data: Vec<i32> = Vec::with_capacity(size);

    let mut ip: usize = main; // instruction pointer register
    let mut fp: usize;        // frame pointer register
    let mut sp: i32 = -1;   // stack pointer register
   
    let mut v: i32;

    while ip < code.len() {
        let mut opcode: VMBytecode = VMBytecode::from(code[ip]);// fetch
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
        vm_exec(hello, 0, 0);
    }

}

