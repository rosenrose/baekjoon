use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let (op, rd, ra, rb_or_c) = (
            input(),
            parse_int(input()),
            parse_int(input()),
            parse_int(input()),
        );

        let asm = format!(
            "{:04b}{}0{rd:03b}{ra:03b}{}",
            match op {
                "ADD" | "ADDC" => 0_u8,
                "SUB" | "SUBC" => 1,
                "MOV" | "MOVC" => 2,
                "AND" | "ANDC" => 3,
                "OR" | "ORC" => 4,
                "NOT" => 5,
                "MULT" | "MULTC" => 6,
                "LSFTL" | "LSFTLC" => 7,
                "LSFTR" | "LSFTRC" => 8,
                "ASFTR" | "ASFTRC" => 9,
                "RL" | "RLC" => 10,
                "RR" | "RRC" => 11,
                _ => 0,
            },
            if op.ends_with('C') { '1' } else { '0' },
            if op.ends_with('C') {
                format!("{rb_or_c:04b}")
            } else {
                format!("{rb_or_c:03b}0")
            },
        );

        writeln!(output, "{asm}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
