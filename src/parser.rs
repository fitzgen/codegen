// Parser for souper tokens

use std::collections::HashMap;
use lexer::{self, Lexer, TokKind, Location, LocatedToken, LocatedError};

pub enum InstKind {
    Var,
    Const,
    UntypedConst,
    Add,
    AddNW,
    AddNSW,
    AddNUW,
    Sub,
    Mul,
    Zext,
    NoneType,
}

pub enum CtonValueDef {
    Result,
    Param,
}

pub enum CtonInstKind {
    Unary,
    UnaryImm,
    Binary,
    BinaryImm,
    Var,
}

pub enum CtonOpcode {
    Iadd,
    IaddImm,
    Var,
}

pub struct Inst<'a> {
    pub kind: InstKind,
    pub lhs: &'a str,
    //pub instWidth: u32,
    // ops: Vec<Inst>
}

pub struct CtonInst<'a> {
    pub valuedef: CtonValueDef,
    pub kind: CtonInstKind,
    pub opcode: CtonOpcode,
    // FIXME: just replica of souper's lhs" do we need this?
    pub lhs: &'a str,
}

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,

    /// Current lookahead token.
    lookahead: Option<TokKind<'a>>,

    /// Location of lookahead.
    loc: Location,

    lex_error: Option<lexer::Error>,

    /// LHS Valname
    lhs_valname: &'a str,

    // hash map of LHS valnames to Index values
    lhsValNames_to_Idx: HashMap<&'a str, u32>,

}

impl<'a> Parser<'a> {
    // Initialize the parser.
    pub fn new(s: &str) -> Parser {
        Parser {
            lex: Lexer::new(s),
            lookahead: None,
            loc: Location { line_num: 0 },
            lex_error: None,
            lhs_valname: "",
            lhsValNames_to_Idx: HashMap::new(),
        }
    }

    fn create_var(&mut self, instkind: InstKind, instname: &'a str) -> Inst<'a> {
        // return the inst struct with details
        // FIXME: add more details later if required
        Inst {
            kind: instkind,
            lhs: instname,
        }
    }

    fn create_inst(&mut self, instkind: InstKind, instname: &'a str) -> Inst<'a> {
        // return the inst struct with details
        // FIXME: add more details later if required
        // Add Ops details too here: Major TODO
        Inst {
            kind: instkind,
            lhs: instname,
        }
    }

    // return inst Kind for the given inst names
    fn get_inst_kind(&mut self, name: &str) -> InstKind {
        match name {
            "var" => InstKind::Var,
            "add" => InstKind::Add,
            "mul" => InstKind::Mul,
            _ => InstKind::NoneType,
        }
    }

    // print token name
    fn get_token_name(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName(lhs)) => println!("ValName "),
            Some(TokKind::Ident(text)) => println!("Ident "),
            Some(TokKind::Comma) => println!("Comma "),
            Some(TokKind::Equal) => println!("Eq "),
            Some(TokKind::Int) => println!("Int "),
            Some(TokKind::Eof) => println!("EOF "),
            Some(TokKind::Error) => println!("Error "),
            Some(TokKind::UntypedInt) => println!("Untypedint "),
            _ => println!("Token type not handled "),
        }
    }

    // returns the current token
    fn consume_token(&mut self) -> Option<TokKind<'a>> {
        let x = self.lookahead.clone();
        match x {
            Some(TokKind::Error) => {
                // do something here to build an error msg
            },
            _=> {
                match self.lex.get_next_token() {
                    Some(Ok(LocatedToken {kind, location})) => {
                        self.lookahead = Some(kind);
                        self.loc = location;
                    },
                    Some(Err(LocatedError{error, errmsg, location})) => {
                        self.lex_error = Some(error);
                        self.loc = location;
                    },
                    _ => {
                        println!("Error: in consume_token(), invalid token type");
                    },
                }
            },
        }
        self.lookahead.clone()
    }

    pub fn is_eof(&mut self) -> bool {
        match self.lookahead {
            Some(TokKind::Eof) => true,
            _ => false,
        }
    }

    fn parse_ops(&mut self) {
        loop {
            //op = self.parse_op();
            self.parse_op();

            // parse_op() already consumed next token, so look
            // for comma token now.
            match self.lookahead {
                Some(TokKind::Comma) => {
                    self.consume_token();
                },
                _ => break,
            }
        }
    }

    //fn parse_ops(&mut self) -> Option<Inst> {
    fn parse_op(&mut self) {
        match self.lookahead {
            Some(TokKind::ValName(lhs)) => {
                // error checking: self.width == 0 => error unexpected width of op

                // Inst I = createInst with inst width, instvalname
                // InstContext IC; IC.getInst()
                // if I is None => error "%<x> is not an inst"

                println!("Op: Valname");
                self.consume_token();

                //return I
            },
            Some(TokKind::Int) => {
                // get the value of const
                // build const inst
                // Inst I = IC.getConst()
                println!("Op: Int");
                self.consume_token();

                // return I
            },
            Some(TokKind::UntypedInt) => {
                // get the value of const
                // build untyped const inst
                // Inst I = IC.getUntypedConst()
                println!("Op: Untyped Int");
                self.consume_token();

                // return I
            },
            _ => {
                // build error
                println!("unexpected token type of Op");
            },
        }
    }
    //}

    fn parse_inst_types(&mut self) -> Option<Inst<'a>> {
        if let Some(TokKind::Ident(text)) = self.lookahead {
            match self.get_inst_kind(text) {
                InstKind::Var => {
                    // TODO: error checking
                    // instwidth == 0 => error "var inst expects atleast width=1"

                    self.consume_token();
                    // Deal with dataflow facts later here!

                    // create Var instruction and return that
                    // self.createVar(instValName, instWidth);
                    println!("Build Var Instruction");
                    // Discuss: Rust study group
                    // Error: cannot borrow `self.lhs_valname` as immutable because `*self` is also borrowed as mutable
                    //Some(self.create_var(InstKind::Var, self.lhs_valname.clone()))

                    let instname = self.lhs_valname.clone();

                    // TODO: HASHMAP: store instnames to index values in hash map
                    //self.lhsValName_to_Idx.insert(instname, )

                    Some(self.create_var(InstKind::Var, instname))
                },
                _ => {
                    let instKind = self.get_inst_kind(text);

                    // Start parsing Ops
                    self.consume_token();
                    self.parse_ops();

                    println!("Build {} instruction", text);
                    // TODO: return the build instruction
                    // IC.getInst(instwidth, instkind, ops)
                    //Some(self.create_inst(instKind, self.lhs_valname.clone()))
                    let instname = self.lhs_valname.clone();
                    
                    // TODO: HASHMAP: store instnames to index values in hash map

                    Some(self.create_inst(instKind, instname))
                },
            }
        } else {
            // return an error here
            println!("Not a valid inst type");
            let inst = None;
            inst
        }
    }

    fn parse_valname_inst(&mut self) -> Option<Inst<'a>> {
        // FIXME: Jubi: Add this info to token struct and get it
        // instwidth = self.width
        // instValName = self.instValname

        // Do error handling:
        // check if %1 (instValName) is already declared as an Inst?
        // context.getInst()

        self.consume_token();

        // Look for Equal token now
        match self.lookahead {
            Some(TokKind::Equal) => {
                self.consume_token();

                // Look for ident tokens like, var; add; phi; etc.
                match self.lookahead {
                    Some(TokKind::Ident(text)) => {
                        // Deal with actual part of inst, like:
                        // var
                        // add %0, %1
                        // phi %0, 1, 2
                        self.parse_inst_types()
                    },
                    _ => {
                        // build error "expected identifier here:Valname -> Eq -> Ident"
                        println!("Error: Expected valname -> Eq -> ??? Ident");
                        // FIXME: here, either build error inst or error return by panic
                        let inst = None;
                        inst
                    },
                }
            },
            _ => {
                // Build error "expected ="
                println!("Error: Expected ValName -> ???? Eq");
                // FIXME: here, either build error inst or error return by panic
                let inst = None;
                inst
            },
        }
    }

    fn parse_ident_inst(&mut self) -> Option<Inst<'a>> {
        // extend this later
        println!("Ident type instructions are not yet handled, like infer, cand, result, pc, blockpc");
        // FIXME: For now, I am simply cnsuming further tokens
        self.consume_token();
        let inst = None;
        inst
    }

    // parse each instruction
    fn parse_inst(&mut self) -> Option<Inst<'a>> {
        // Instructions start either with valname or Ident
        // Example:
        // %1:i32 = .... 
        // cand ... , infer ... , result ...
        // pc ... , blockpc ... , 

        match self.lookahead {
            Some(TokKind::ValName(lhs)) => {
                self.lhs_valname = lhs;
                self.parse_valname_inst()
            },
            Some(TokKind::Ident(text)) => {
                self.parse_ident_inst()
            },
            _ => {
                println!("Error: Instruction either start with ValName token or Ident token");
                // FIXME: Jubi: Build an error
                let inst = None;
                inst
            },
        }
    }
}


pub fn parse(text: &str) {
    let mut p = Parser::new(text);

    p.consume_token();

    // FIXMEL we want a ret value from parse_line() to
    // be used later for code gen purpose

    let mut insts: Vec<Option<Inst>> = Vec::new();
    loop {
        match p.lookahead {
            Some(TokKind::Eof) => break,
            _ => {
                let inst = p.parse_inst();
                insts.push(inst);

                //FIXME: Do we need this error checking with match?
                //match inst {
                //    Some(Inst {kind, lhs} ) => {
                //        println!("Inst LHS at the end ======= {}", lhs);
                //        // Push the inst parsed in a vector here
                //        let i = inst.clone();
                //        insts.push(i);
                //    },
                //    _ => {
                //        println!("Invalid Inst parsed");
                //    },
                //}
            },
        }
    }
    //lowering_souper_isa_to_cton_isa(insts);
    let ctonInsts = lowering_souper_isa_to_cton_isa(insts);
    for c in ctonInsts {
        println!("cton inst created: {:?}, {:?}, {:?}", getCtonValDefName(c.valuedef), getCtonOpCodeName(c.opcode), c.lhs)
    }
}

pub fn getCtonOpCodeName(opcode: CtonOpcode) {
    match opcode {
        CtonOpcode::Iadd => println!("Cton: Iadd"),
        _ => println!("Cton: other type yet to be handled"),
    }
}

pub fn getCtonValDefName(vdef: CtonValueDef) {
    match vdef {
        CtonValueDef::Result => println!("Cton: ValueDef"),
        CtonValueDef::Param => println!("Cton: Param"),
        _ => println!("Cton: No such value def types"),
    }
}

pub fn mapping_souper_to_cton_isa(souper_inst: Option<Inst>) -> CtonInst {
    match souper_inst {
        Some(Inst{kind, lhs}) => {
            match kind {
                InstKind::Add => {
                    CtonInst{
                        valuedef: CtonValueDef::Result,
                        kind: CtonInstKind::Binary,
                        opcode: CtonOpcode::Iadd,
                        lhs: lhs,
                    }
                },
                InstKind::Var => {
                    CtonInst{
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        lhs: lhs,
                    }
                },
                _ => {
                    CtonInst{
                        valuedef: CtonValueDef::Param,
                        kind: CtonInstKind::Var,
                        opcode: CtonOpcode::Var,
                        lhs: lhs,
                    }
                },
            }
        },
        _ => {
            CtonInst{
                valuedef: CtonValueDef::Param,
                kind: CtonInstKind::Var,
                opcode: CtonOpcode::Var,
                lhs: "",
            }
        },
    }
}

//fn lowering_souper_isa_to_cton_isa(souper_insts: Vec<Inst>) {
fn lowering_souper_isa_to_cton_isa(souper_insts: Vec<Option<Inst>>) -> Vec<CtonInst> {
    let mut cton_insts: Vec<CtonInst> = Vec::new();
    for souper_inst in souper_insts {
        // get the mapping souper ISA to cretonne ISA
        // And, insert each cton inst to a new vec<cton_inst>
        // add more details to cton inst structure:
        // name, binary/unary instruction data, 
        let cton_inst = mapping_souper_to_cton_isa(souper_inst);
        cton_insts.push(cton_inst);
    }
    cton_insts
}
