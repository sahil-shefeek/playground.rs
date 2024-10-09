use std::{env, fs, path::Path, str};

#[derive(Debug)]
struct Token<'a> {
    label: &'a str,
    opcode: &'a str,
    operand: &'a str,
}

impl<'a> Token<'a> {
    fn new(raw_input: Vec<&'a str>) -> Self {
        Self {
            label: raw_input[0],
            opcode: raw_input[1],
            operand: raw_input[2],
        }
    }

    fn has_label_field(&self) -> bool {
        if self.label.is_empty() || self.label.eq("**") {
            return false;
        }
        true
    }
}

struct SymTabEntries {
    name: String,
    value: i32,
}

impl SymTabEntries {
    fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }
}

fn search_symtab(symtab: &[SymTabEntries], key: &str) -> bool {
    let mut i = 0;
    while i < symtab.len() {
        if symtab[i].name.eq(key) {
            return true;
        }
        i += 1;
    }
    false
}

struct OpTabEntries {
    mnemonic: String,
    machine_code: i32,
}

impl OpTabEntries {
    fn new(mnemonic: String, machine_code: i32) -> Self {
        Self {
            mnemonic,
            machine_code,
        }
    }

    fn from(file_path: &Path) -> Vec<Self> {
        let mut entries: Vec<OpTabEntries> = vec![];
        let optab_lines = read_lines(file_path);
        for line in optab_lines {
            let token: Vec<&str> = line.split("\t").collect();
            if token[0].contains("END") {
                continue;
            };
            if let Ok(machine_code) = i32::from_str_radix(token[1], 16) {
                entries.push(OpTabEntries::new(String::from(token[0]), machine_code));
            } else {
                panic!("Falied to parse machine code")
            }
        }
        entries
    }
}

fn search_optab(optab: &[OpTabEntries], key: &str) -> bool {
    let mut i = 0;
    while i < optab.len() {
        if optab[i].mnemonic.eq(key) {
            return true;
        }
        i += 1;
    }
    false
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let input_src_path = Path::new("input/input.txt");
    let optab_path = Path::new("input/optab.txt");
    let ifile_path = Path::new("output/ifile.txt");
    let mut ifile_string = String::new();
    let lines = read_lines(input_src_path);
    let mut locctr = 0;
    let mut symtab: Vec<SymTabEntries> = vec![];
    let optab: Vec<OpTabEntries> = OpTabEntries::from(optab_path);
    for line in lines {
        let token = Token::new(line.split("\t").collect());
        if token.opcode.eq("START") {
            locctr = token.operand.parse().expect("Error converting LOCCTR");
            ifile_string = ifile_string + "**\t" + { token.label } + "\t" + token.operand + "\n";
            continue;
        }
        if token.opcode.ne("END") {
            if token.has_label_field() {
                if search_symtab(&symtab, token.label) {
                    panic!("Duplicate Symbol")
                } else {
                    symtab.push(SymTabEntries::new(String::from(token.label), locctr));
                }
            }
            if search_optab(&optab, token.opcode) {
                locctr += 3;
            } else {
                match token.opcode {
                    "WORD" => locctr += 3,
                    "RESW" => {
                        locctr += 3 * token
                            .operand
                            .parse::<i32>()
                            .expect("Error converting operand to int")
                    }

                    "RESB" => {
                        locctr += token
                            .operand
                            .parse::<i32>()
                            .expect("Error converting operand to int")
                    }
                    "BYTE" => locctr += token.operand.len() as i32,
                    _ => panic!("Invalid operation code!"),
                }
            }
        }
        ifile_string = ifile_string
            + locctr.to_string().as_str()
            + "\t"
            + token.label
            + "\t"
            + token.opcode
            + "\t"
            + token.operand
            + "\n";
    }

    if let Ok(()) = fs::write(ifile_path, ifile_string.as_bytes()) {
        println!("Write Successful!")
    } else {
        println!("Write failed!")
    }
}

fn read_lines(file_path: &Path) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
