use nom::character::complete::{u64, u8};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, tuple};
use nom::{bytes::complete::tag, IResult};
use util;

util::main![pt1, pt2];

#[derive(Debug)]
enum OpCode {
    ADV = 0, // divide reg A by combo operand, trunc to int, store in reg A
    BXL = 1, // combo operand mod 8, store in reg B
    BST = 2, // bitwise XOR reg B and literal operand, store in reg B
    JNZ = 3, // if A is not 0 jump to pos of literal operand, skip incrementing instr pointer
    BXC = 4, // bitwise XOR reg B and reg C, store in reg B
    OUT = 5, // combo operand mod 8, output value (multiples are comma separated)
    BDV = 6, // divide reg A by combo operand, trunc to int, store in reg B
    CDV = 7, // divide reg A by combo operand, trunc to int, store in reg C
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            x if x == OpCode::ADV as u8 => Ok(OpCode::ADV),
            x if x == OpCode::BXL as u8 => Ok(OpCode::BXL),
            x if x == OpCode::BST as u8 => Ok(OpCode::BST),
            x if x == OpCode::JNZ as u8 => Ok(OpCode::JNZ),
            x if x == OpCode::BXC as u8 => Ok(OpCode::BXC),
            x if x == OpCode::OUT as u8 => Ok(OpCode::OUT),
            x if x == OpCode::BDV as u8 => Ok(OpCode::BDV),
            x if x == OpCode::CDV as u8 => Ok(OpCode::CDV),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Op {
    opc: OpCode,
    opnd: u8,
}

#[derive(Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    iptr: usize,
    program: Vec<Op>,
    raw: Vec<u8>,
}

impl State {
    fn get_combo_opnd(&self, op: &Op) -> Result<u64, anyhow::Error> {
        match op.opnd {
            x if x <= 3 => Ok(x as u64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            x => Err(anyhow::anyhow!(
                "Operand cannot be converted into combo operand: {}",
                x
            )),
        }
    }
    fn run(&mut self) -> Result<Vec<u8>, anyhow::Error> {
        let mut output = vec![];

        while let Some(instr) = self.program.get(self.iptr) {
            match instr.opc {
                OpCode::ADV => {
                    let copnd = self.get_combo_opnd(instr)?;
                    self.a = self.a / (1 << copnd);
                }
                OpCode::BXL => {
                    self.b ^= instr.opnd as u64;
                }
                OpCode::BST => {
                    let copnd = self.get_combo_opnd(instr)?;
                    self.b = copnd % 8;
                }
                OpCode::JNZ => {
                    if self.a != 0 {
                        self.iptr = instr.opnd as usize;
                        continue;
                    }
                }
                OpCode::BXC => {
                    self.b ^= self.c;
                }
                OpCode::OUT => {
                    let copnd = self.get_combo_opnd(instr)?;
                    let out = (copnd % 8) as u8;
                    output.push(out);
                }
                OpCode::BDV => {
                    let copnd = self.get_combo_opnd(instr)?;
                    self.b = self.a / (1 << copnd);
                }
                OpCode::CDV => {
                    let copnd = self.get_combo_opnd(instr)?;
                    self.c = self.a / (1 << copnd);
                }
            }
            self.iptr += 1;
        }

        Ok(output)
    }
    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.iptr = 0;
    }
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (remainder, (a, b, c, raw)): (&str, (u64, u64, u64, Vec<u8>)) = tuple((
        delimited(tag("Register A: "), u64, tag("\n")),
        delimited(tag("Register B: "), u64, tag("\n")),
        delimited(tag("Register C: "), u64, tag("\n\n")),
        preceded(tag("Program: "), separated_list0(tag(","), u8)),
    ))(input)?;
    let program = raw
        .chunks(2)
        .filter_map(|sl| {
            if sl.len() != 2 {
                return None;
            }
            Some(Op {
                opc: (sl[0]).try_into().unwrap(),
                opnd: sl[1],
            })
        })
        .collect();

    Ok((
        remainder,
        State {
            a,
            b,
            c,
            iptr: 0,
            program,
            raw,
        },
    ))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut st: State = util::parse::with_nom(&path, parse_state)?;

    let output = st.run()?;
    print!("{}", output[0]);
    for out in output.iter().skip(1) {
        print!(",{}", out);
    }
    println!();

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut st: State = util::parse::with_nom(&path, parse_state)?;

    let mut a = 1;
    st.reset(a);
    while let Ok(out) = st.run() {
        let len_diff = st.raw.len() - out.len();
        let tail = st.raw.iter().skip(len_diff);
        if out.iter().eq(tail) {
            if len_diff == 0 {
                break;
            }
            a <<= 3; // trial and error discovery, pattern on powers of 8.
        } else {
            a += 1;
        }
        st.reset(a);
    }
    println!("Initial value of Register A to produce self: {}", a);

    Ok(())
}
