#![allow(non_snake_case)]

use core::panic;
use std::{collections::HashMap, vec};

use crate::tokenizer::{Token, TokenType::*, Tokenizer};

#[derive(Debug, Clone, Copy)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Debug, Clone, Copy)]
pub enum DeclareType {
    Static,
    Field,
    Arg,
    Var,
}
impl DeclareType {
    fn to_string(&self) -> String{
        match self{
            DeclareType::Static => "static".to_owned(),
            DeclareType::Field => "this".to_owned(),
            DeclareType::Arg => "argument".to_owned(),
            DeclareType::Var => "local".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
    Void,
}

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    var_dec: Vec<ClassVarDec>,
    sub_dec: Vec<SubroutineDec>,
}
#[derive(Debug, Clone)]
struct ClassVarDec {
    declare_type: DeclareType,
    type_type: Type,
    variable_names: Vec<String>,
}
#[derive(Debug, Clone)]
struct SubroutineDec {
    subroutine_type: SubroutineType,
    return_type: Type,
    name: String,
    parameter_list: ParameterList,
    body: SubroutineBody,
}
#[derive(Debug, Clone)]
struct ParameterList {
    list: Vec<(Type, String)>,
}
#[derive(Debug, Clone)]
struct SubroutineBody {
    variable_declaration: Vec<VarDec>,
    body: Vec<Statement>,
}
#[derive(Debug, Clone)]
struct VarDec {
    declare_type: DeclareType,
    type_type: Type,
    variable_names: Vec<String>,
}
#[derive(Debug, Clone)]
struct SubroutineCall {
    obj_name: Option<String>,
    subroutine_name: String,
    arguments: ExpressionList, // ExpressionList
}

// Term (Op Term)*
#[derive(Debug, Clone)]
struct Expression {
    intial_term: Term,
    afterwards: Vec<(String, Term)>,
}
#[derive(Debug, Clone)]
enum Term {
    IntegerConstant(u64),
    StringConstant(String),
    KeywordConstant(String),
    VarName(String),
    ArrayAccess {
        array_name: String,
        index: Box<Expression>,
    },
    SubroutineCall(SubroutineCall),
    Expression(Box<Expression>),
    UnaryOp {
        op: String,
        term: Box<Term>,
    },
}

#[derive(Debug, Clone)]
struct ExpressionList {
    list: Vec<Expression>,
}

#[derive(Debug, Clone)]
enum Statement {
    Let(Let),
    If(If),
    While(While),
    Do(Do),
    Return(Return),
}
#[derive(Debug, Clone)]
struct Let {
    var_name: String,
    array_acces: Option<Expression>, // has to be a Expression
    bind_to: Expression,             // has to be a Expression
}
#[derive(Debug, Clone)]
struct If {
    condition: Expression, // has to be a Expression
    if_true: Vec<Statement>,
    else_part: Option<Vec<Statement>>,
}
#[derive(Debug, Clone)]
struct While {
    condition: Expression, // has to be a Expression
    if_true: Vec<Statement>,
}
#[derive(Debug, Clone)]
struct Do {
    call: SubroutineCall,
}
#[derive(Debug, Clone)]
struct Return {
    value: Option<Expression>, // must be Expression
}

// #[derive(Debug,Clone)]
// pub enum TokenOrParseNode {
//     Token(Token),
//     ParseNode(ParseNode),
// }

// #[derive(Debug,Clone)]
// pub struct ParseNode {
//     pub node_type: ParseNodeType,
//     pub body: Vec<TokenOrParseNode>,
// }

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn from_tokenizer(tokenizer: Tokenizer) -> Parser {
        Self { tokenizer }
    }
    pub fn compileClass(&mut self) -> Class {
        let mut class_node = Class {
            name: String::new(),
            var_dec: vec![],
            sub_dec: vec![],
        };

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Keyword, "class");

        class_node.name = self
            .tokenizer
            .expect(crate::tokenizer::TokenType::Identifier)
            .string_repr;

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, "{");

        // classVarDec*
        while self.tokenizer.current().t_type == Keyword
            && (self.tokenizer.current().string_repr == "static"
                || self.tokenizer.current().string_repr == "field")
        {
            class_node.var_dec.push(self.compileClassVarDec());
        }

        // subroutineDec*
        while self.tokenizer.current().t_type == Keyword
            && (self.tokenizer.current().string_repr == "constructor"
                || self.tokenizer.current().string_repr == "function"
                || self.tokenizer.current().string_repr == "method")
        {
            class_node.sub_dec.push(self.compileSubroutineDec());
        }

        self.tokenizer.expect_and_string(Symbol, "}");

        class_node
    }
    fn compileClassVarDec(&mut self) -> ClassVarDec {
        let mut node_temp = ClassVarDec {
            declare_type: DeclareType::Field, // To Be Changed
            type_type: Type::Int,             // To Be Changed
            variable_names: vec![],           // To Be Changed
        };
        let declare_type = match self.tokenizer.expect(Keyword).string_repr.as_str() {
            "static" => DeclareType::Static,
            "field" => DeclareType::Field,
            _ => unreachable!(),
        };
        node_temp.declare_type = declare_type;

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword
            && !(type_of_dec.string_repr == "int"
                || type_of_dec.string_repr == "char"
                || type_of_dec.string_repr == "boolean")
        {
            panic!("Expected int char or boolean as type if it is a keyword!");
        } else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier {
            node_temp.type_type = match type_of_dec.string_repr.as_str() {
                "int" => Type::Int,
                "char" => Type::Char,
                "boolean" => Type::Boolean,
                custom_type => Type::ClassName(custom_type.to_string()),
            };
        } else {
            panic!("Expected int,char,boolean or Identifier as type in compileClassVarDec!");
        }
        node_temp
            .variable_names
            .push(self.tokenizer.expect(Identifier).string_repr);

        while self.tokenizer.current().t_type == Symbol
            && self.tokenizer.current().string_repr == ","
        {
            self.tokenizer.advance();
            node_temp
                .variable_names
                .push(self.tokenizer.expect(Identifier).string_repr);
        }

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, ";");

        node_temp
    }
    fn compileSubroutineDec(&mut self) -> SubroutineDec {
        let mut node_temp = SubroutineDec {
            subroutine_type: match self.tokenizer.expect(Keyword).string_repr.as_str() {
                "constructor" => SubroutineType::Constructor,
                "method" => SubroutineType::Method,
                "function" => SubroutineType::Function,
                _ => unreachable!(),
            },
            name: String::new(),
            parameter_list: ParameterList { list: vec![] },
            body: SubroutineBody {
                variable_declaration: vec![],
                body: vec![],
            },
            return_type: Type::Void,
        };

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword
            && !(type_of_dec.string_repr == "int"
                || type_of_dec.string_repr == "char"
                || type_of_dec.string_repr == "boolean"
                || type_of_dec.string_repr == "void")
        {
            panic!("Expected int char void or boolean as type if it is a keyword!");
        } else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier {
            node_temp.return_type = match type_of_dec.string_repr.as_str() {
                "int" => Type::Int,
                "char" => Type::Char,
                "boolean" => Type::Boolean,
                "void" => Type::Void,
                custom_type => Type::ClassName(custom_type.to_string()),
            };
        } else {
            panic!("Expected int,char,boolean,void or Identifier as type in compileSubroutineDec!");
        }

        node_temp.name = self.tokenizer.expect(Identifier).string_repr;

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, "(");

        if !(self.tokenizer.current().t_type == Symbol
            && self.tokenizer.current().string_repr == ")")
        {
            node_temp.parameter_list = self.compileParameterList();
        }

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, ")");

        node_temp.body = self.compileSubroutineBody();

        node_temp
    }
    fn compileParameterList(&mut self) -> ParameterList {
        let mut node_temp = ParameterList { list: vec![] };

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword
            && !(type_of_dec.string_repr == "int"
                || type_of_dec.string_repr == "char"
                || type_of_dec.string_repr == "boolean")
        {
            panic!("Expected int char or boolean as type if it is a keyword!");
        } else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier {
            node_temp.list.push((
                match type_of_dec.string_repr.as_str() {
                    "int" => Type::Int,
                    "char" => Type::Char,
                    "boolean" => Type::Boolean,
                    "void" => Type::Void,
                    custom_type => Type::ClassName(custom_type.to_string()),
                },
                self.tokenizer.expect(Identifier).string_repr,
            ));
        } else {
            panic!("Expected int,char,boolean or Identifier as type in compileParameterList!");
        }

        while self.tokenizer.current().t_type == Symbol
            && self.tokenizer.current().string_repr == ","
        {
            self.tokenizer.advance();

            let type_of_dec = self.tokenizer.advance();
            if type_of_dec.t_type == Keyword
                && !(type_of_dec.string_repr == "int"
                    || type_of_dec.string_repr == "char"
                    || type_of_dec.string_repr == "boolean")
            {
                panic!("Expected int char or boolean as type if it is a keyword!");
            } else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier {
                node_temp.list.push((
                    match type_of_dec.string_repr.as_str() {
                        "int" => Type::Int,
                        "char" => Type::Char,
                        "boolean" => Type::Boolean,
                        "void" => Type::Void,
                        custom_type => Type::ClassName(custom_type.to_string()),
                    },
                    self.tokenizer.expect(Identifier).string_repr,
                ));
            } else {
                panic!("Expected int,char,boolean or Identifier as type in compileParameterList!");
            }
        }

        node_temp
    }
    fn compileSubroutineBody(&mut self) -> SubroutineBody {
        let mut node_temp = SubroutineBody {
            variable_declaration: vec![],
            body: vec![],
        };

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, "{");

        // varDec*
        while self.tokenizer.current().t_type == Keyword
            && self.tokenizer.current().string_repr == "var"
        {
            node_temp.variable_declaration.push(self.compileVarDec());
        }

        // statements
        node_temp.body = self.compileStatements();

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, "}");
        node_temp
    }
    fn compileVarDec(&mut self) -> VarDec {
        let mut node_temp = VarDec {
            declare_type: DeclareType::Var,
            type_type: Type::Int,
            variable_names: vec![],
        };
        self.tokenizer.expect(Keyword);

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword
            && !(type_of_dec.string_repr == "int"
                || type_of_dec.string_repr == "char"
                || type_of_dec.string_repr == "boolean")
        {
            panic!("Expected int char or boolean as type if it is a keyword!");
        } else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier {
            node_temp.type_type = match type_of_dec.string_repr.as_str() {
                "int" => Type::Int,
                "char" => Type::Char,
                "boolean" => Type::Boolean,
                "void" => Type::Void,
                custom_type => Type::ClassName(custom_type.to_string()),
            };
        } else {
            panic!("Expected int,char,boolean or Identifier as type in compileClassVarDec!");
        }
        node_temp
            .variable_names
            .push(self.tokenizer.expect(Identifier).string_repr);

        while self.tokenizer.current().t_type == Symbol
            && self.tokenizer.current().string_repr == ","
        {
            self.tokenizer.advance();
            node_temp
                .variable_names
                .push(self.tokenizer.expect(Identifier).string_repr);
        }

        self.tokenizer
            .expect_and_string(crate::tokenizer::TokenType::Symbol, ";");

        node_temp
    }
    fn compileStatements(&mut self) -> Vec<Statement> {
        let mut temp = vec![];
        // statement*
        // either let,if,while,do,return
        while self.tokenizer.current().t_type == Keyword
            && (self.tokenizer.current().string_repr == "let"
                || self.tokenizer.current().string_repr == "if"
                || self.tokenizer.current().string_repr == "while"
                || self.tokenizer.current().string_repr == "do"
                || self.tokenizer.current().string_repr == "return")
        {
            match self.tokenizer.current().string_repr.as_str() {
                "let" => {
                    temp.push(self.compileLet());
                }
                "if" => {
                    temp.push(self.compileIf());
                }
                "while" => {
                    temp.push(self.compileWhile());
                }
                "do" => {
                    temp.push(self.compileDo());
                }
                "return" => {
                    temp.push(self.compileReturn());
                }
                _ => unreachable!(),
            }
        }

        temp
    }
    fn compileLet(&mut self) -> Statement {
        let mut node_temp = Let {
            var_name: String::new(),
            array_acces: None,
            bind_to: Expression {
                intial_term: Term::IntegerConstant(7),
                afterwards: vec![],
            },
        };

        self.tokenizer.expect(Keyword);

        node_temp.var_name = self.tokenizer.expect(Identifier).string_repr;

        if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "["
        {
            self.tokenizer.advance();
            node_temp.array_acces = Some(self.compileExpression());

            self.tokenizer.expect_and_string(Symbol, "]");
        }

        self.tokenizer.expect_and_string(Symbol, "=");

        node_temp.bind_to = self.compileExpression();

        self.tokenizer.expect_and_string(Symbol, ";");
        Statement::Let(node_temp)
    }
    fn compileIf(&mut self) -> Statement {
        let mut node_temp = If {
            condition: Expression {
                intial_term: Term::IntegerConstant(7),
                afterwards: vec![],
            },
            if_true: vec![],
            else_part: None,
        };

        self.tokenizer.expect(Keyword);

        self.tokenizer.expect_and_string(Symbol, "(");

        node_temp.condition = self.compileExpression();

        self.tokenizer.expect_and_string(Symbol, ")");
        self.tokenizer.expect_and_string(Symbol, "{");

        node_temp.if_true = self.compileStatements();

        self.tokenizer.expect_and_string(Symbol, "}");

        if self.tokenizer.current().t_type == Keyword
            && self.tokenizer.current().string_repr == "else"
        {
            self.tokenizer.advance();

            self.tokenizer.expect_and_string(Symbol, "{");

            node_temp.else_part = Some(self.compileStatements());

            self.tokenizer.expect_and_string(Symbol, "}");
        }

        Statement::If(node_temp)
    }
    fn compileWhile(&mut self) -> Statement {
        let mut node_temp = While {
            condition: Expression {
                intial_term: Term::IntegerConstant(7),
                afterwards: vec![],
            },
            if_true: vec![],
        };
        self.tokenizer.expect(Keyword);

        self.tokenizer.expect_and_string(Symbol, "(");

        node_temp.condition = self.compileExpression();

        self.tokenizer.expect_and_string(Symbol, ")");
        self.tokenizer.expect_and_string(Symbol, "{");

        node_temp.if_true = self.compileStatements();

        self.tokenizer.expect_and_string(Symbol, "}");

        Statement::While(node_temp)
    }
    fn compileDo(&mut self) -> Statement {
        let mut node_temp = Do {
            call: SubroutineCall {
                obj_name: None,
                subroutine_name: String::new(),
                arguments: ExpressionList { list: vec![] },
            },
        };
        self.tokenizer.expect(Keyword);

        node_temp.call.subroutine_name = self.tokenizer.expect(Identifier).string_repr;
        if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "."
        {
            self.tokenizer.expect(Symbol);

            node_temp.call.obj_name = Some(node_temp.call.subroutine_name);
            node_temp.call.subroutine_name = self.tokenizer.expect(Identifier).string_repr;
        }

        self.tokenizer.expect_and_string(Symbol, "(");
        node_temp.call.arguments = self.compileExpressionList();

        self.tokenizer.expect_and_string(Symbol, ")");

        self.tokenizer.expect_and_string(Symbol, ";");

        Statement::Do(node_temp)
    }
    fn compileReturn(&mut self) -> Statement {
        let mut node_temp = Return { value: None };
        self.tokenizer.expect(Keyword);

        if !(self.tokenizer.current().t_type == Symbol
            && self.tokenizer.current().string_repr == ";")
        {
            node_temp.value = Some(self.compileExpression());
        }

        self.tokenizer.expect_and_string(Symbol, ";");
        Statement::Return(node_temp)
    }
    fn compileExpression(&mut self) -> Expression {
        let mut node_temp = Expression {
            intial_term: self.compileTerm(),
            afterwards: vec![],
        };

        while self.tokenizer.current().t_type == Symbol
            && (self.tokenizer.current().string_repr == "+"
                || self.tokenizer.current().string_repr == "-"
                || self.tokenizer.current().string_repr == "*"
                || self.tokenizer.current().string_repr == "/"
                || self.tokenizer.current().string_repr == "&"
                || self.tokenizer.current().string_repr == "|"
                || self.tokenizer.current().string_repr == "<"
                || self.tokenizer.current().string_repr == ">"
                || self.tokenizer.current().string_repr == "=")
        {
            node_temp
                .afterwards
                .push((self.tokenizer.advance().string_repr, self.compileTerm()));
        }

        node_temp
    }
    fn compileTerm(&mut self) -> Term {
        let mut node_temp = Term::IntegerConstant(7);
        match self.tokenizer.current().t_type {
            Keyword => {
                // either true false null or this
                if self.tokenizer.current().string_repr == "true"
                    || self.tokenizer.current().string_repr == "false"
                    || self.tokenizer.current().string_repr == "null"
                    || self.tokenizer.current().string_repr == "this"
                {
                    node_temp = Term::KeywordConstant(self.tokenizer.advance().string_repr);
                } else {
                    panic!("keyword in Term other than true false or null or this arent possible! found: {:?}",self.tokenizer.current());
                }
            }
            Symbol => {
                // either expression in paratheses or unaryOp
                if self.tokenizer.current().string_repr == "-"
                    || self.tokenizer.current().string_repr == "~"
                {
                    node_temp = Term::UnaryOp {
                        op: self.tokenizer.advance().string_repr,
                        term: Box::new(self.compileTerm()),
                    };
                } else if self.tokenizer.current().string_repr == "(" {
                    self.tokenizer.advance();
                    node_temp = Term::Expression(Box::new(self.compileExpression()));

                    self.tokenizer.expect_and_string(Symbol, ")");
                } else {
                    panic!(
                        "Symbol in Term other than ~,- or (  arent possible! found: {:?}",
                        self.tokenizer.current()
                    );
                }
            }
            Identifier => {
                let token = self.tokenizer.advance();
                if self.tokenizer.current().t_type == Symbol
                    && self.tokenizer.current().string_repr == "["
                {
                    self.tokenizer.expect_and_string(Symbol, "[");

                    node_temp = Term::ArrayAccess {
                        array_name: token.string_repr,
                        index: Box::new(self.compileExpression()),
                    };

                    self.tokenizer.expect_and_string(Symbol, "]");
                } else if self.tokenizer.current().t_type == Symbol
                    && self.tokenizer.current().string_repr == "("
                {
                    self.tokenizer.expect_and_string(Symbol, "(");
                    node_temp = Term::SubroutineCall(SubroutineCall {
                        obj_name: None,
                        subroutine_name: token.string_repr,
                        arguments: self.compileExpressionList(),
                    });

                    self.tokenizer.expect_and_string(Symbol, ")");
                } else if self.tokenizer.current().t_type == Symbol
                    && self.tokenizer.current().string_repr == "."
                {
                    self.tokenizer.expect(Symbol);
                    let mut temp = SubroutineCall {
                        obj_name: Some(token.string_repr),
                        subroutine_name: self.tokenizer.expect(Identifier).string_repr,
                        arguments: ExpressionList { list: vec![] },
                    };
                    self.tokenizer.expect_and_string(Symbol, "(");

                    temp.arguments = self.compileExpressionList();

                    self.tokenizer.expect_and_string(Symbol, ")");

                    node_temp = Term::SubroutineCall(temp);
                } else {
                    node_temp = Term::VarName(token.string_repr);
                }
            }
            IntegerConstant => {
                node_temp =
                    Term::IntegerConstant(self.tokenizer.advance().string_repr.parse().unwrap());
            }
            StringConstant => {
                node_temp =
                    Term::StringConstant(self.tokenizer.advance().string_repr.parse().unwrap());
            }
        }

        node_temp
    }
    fn compileExpressionList(&mut self) -> ExpressionList {
        let mut node_temp = ExpressionList { list: vec![] };

        // only if there is a expression list
        if self.tokenizer.current().string_repr != ")" {
            node_temp.list.push(self.compileExpression());

            while self.tokenizer.current().t_type == Symbol
                && self.tokenizer.current().string_repr == ","
            {
                self.tokenizer.advance();
                node_temp.list.push(self.compileExpression());
            }
        }

        node_temp
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    v_type: Type,
    kind: DeclareType,
    number: u64,
}

#[derive(Debug, Clone)]
struct Scope {
    table: HashMap<String, Variable>,
    static_counter: u64,
    field_counter: u64,
    arg_counter: u64,
    var_counter: u64,
}
impl Scope{
    fn insert(&mut self,var_name: String,v_type: Type,declare_type: DeclareType){
        self.table.insert(var_name.clone(), Variable { name: var_name, v_type, kind: declare_type, number: match declare_type {
            DeclareType::Static => self.static_counter,
            DeclareType::Field => self.field_counter,
            DeclareType::Arg => self.arg_counter,
            DeclareType::Var => self.var_counter,
        } });
        let num: &mut u64 = match declare_type {
            DeclareType::Static => &mut self.static_counter,
            DeclareType::Field => &mut self.field_counter,
            DeclareType::Arg => &mut self.arg_counter,
            DeclareType::Var => &mut self.var_counter,
        };
        *num += 1;
    }
}

pub struct CodeGenerator {
    class_table: Scope,
    subroutine_table: Scope,
    label_counter: u64,
}
impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            class_table: Scope {
                table: HashMap::new(),
                static_counter: 0,
                field_counter: 0,
                var_counter: 0,
                arg_counter: 0,
            },
            subroutine_table: Scope {
                table: HashMap::new(),
                static_counter: 0,
                field_counter: 0,
                var_counter: 0,
                arg_counter: 0,
            },
            label_counter: 0,
        }
    }
    pub fn to_vm_code(&mut self, node: Class) -> String {
        let mut buf: Vec<String> = vec![];

        for class_level_var in node.var_dec.clone(){
            for name in class_level_var.variable_names{
                self.class_table.insert(name,class_level_var.type_type.clone(),class_level_var.declare_type);
            }
        }
        for sub in node.sub_dec{
            for arg in sub.parameter_list.list{
                self.subroutine_table.insert(arg.1, arg.0, DeclareType::Arg);
            }
            for var_dec in sub.body.variable_declaration{
                for name in var_dec.variable_names{
                    self.subroutine_table.insert(name, var_dec.type_type.clone(), var_dec.declare_type);
                }
            }
            // function vm declaration
            buf.push(format!("function {}.{} {}",node.name,sub.name,self.subroutine_table.var_counter));


            match sub.subroutine_type{
                SubroutineType::Constructor => {
                    buf.push(format!("push constant {}",self.class_table.field_counter)); // size of the class
                    buf.push("call Memory.alloc 1".to_string()); // alloc object
                    buf.push("pop pointer 0".to_string());
                },
                SubroutineType::Method => {
                    buf.push("push argument 0".to_string()); // need to set this with implicit argument 0 on methods
                    buf.push("pop pointer 0".to_string());
                },
                SubroutineType::Function => {
                    // nothing needed here i think
                },
            }
            // handle body:
            for statement in sub.body.body{
                buf.push(self.statement_to_vm_code(statement));
            }

            // reset subroutine table
            self.subroutine_table = Scope {
                table: HashMap::new(),
                static_counter: 0,
                field_counter: 0,
                var_counter: 0,
                arg_counter: 0,
            };
        }

        buf.join("\n")
    }
    fn statement_to_vm_code(&mut self,node: Statement) -> String{
        let mut buf: Vec<String> = vec![];

        match node{
            Statement::Let(let_statement) => {
                let var_to_assign_to;
                let num_temp = self.subroutine_table.table.get(&let_statement.var_name);
                if let Some(var) = num_temp{
                    var_to_assign_to = var.clone();
                }else if let Some(var) = self.class_table.table.get(&let_statement.var_name){
                    var_to_assign_to = var.clone();
                } else {
                    panic!("var not found in class or subroutine to assign to with let statement");
                }

                if let_statement.array_acces.is_none(){
                    buf.push(self.expression_to_vm_code(let_statement.bind_to));
                    buf.push(format!("pop {} {}",var_to_assign_to.kind.to_string(),var_to_assign_to.number));
                }else{
                    // Array access TODO
                    todo!()
                }
            },
            Statement::If(if_statement) => {
                let label1 = format!("Label_{}",&self.label_counter);
                self.label_counter += 1;
                let label2 = format!("Label_{}",&self.label_counter);
                self.label_counter += 1;

                buf.push(self.expression_to_vm_code(if_statement.condition));

                buf.push("not".to_string());

                buf.push(format!("if-goto {}",label1));

                for statement in if_statement.if_true{
                    buf.push(self.statement_to_vm_code(statement));
                }
                buf.push(format!("goto {}",label2));
                buf.push(format!("label {}",label1));

                if let Some(else_statements) = if_statement.else_part{
                    for statement in else_statements{
                        buf.push(self.statement_to_vm_code(statement));
                    }
                }

                buf.push(format!("label {}",label2));
            },
            Statement::While(while_statement) => {
                // label L1
                // compiled (expression)
                // not
                // if-goto L2
                // compiled (statements)
                // goto L1
                // label L2

                let label1 = format!("Label_{}",&self.label_counter);
                self.label_counter += 1;
                let label2 = format!("Label_{}",&self.label_counter);
                self.label_counter += 1;

                buf.push(format!("label {}",label1));

                buf.push(self.expression_to_vm_code(while_statement.condition));

                buf.push("not".to_string());

                buf.push(format!("if-goto {}",label2));

                for statement in while_statement.if_true{
                    buf.push(self.statement_to_vm_code(statement));
                }
                buf.push(format!("goto {}",label1));

                buf.push(format!("label {}",label2));
            },
            Statement::Do(do_statement) => {
                let subroutine_call = do_statement.call;

                buf.push(self.sub_call_to_vm_code(subroutine_call));

                // then pop the return value away!
                buf.push("pop temp 0".to_owned());
            },
            Statement::Return(return_statement) => {
                if let Some(return_expr) = return_statement.value{
                    buf.push(self.expression_to_vm_code(return_expr));
                    buf.push("return".to_owned());
                }else{
                    buf.push("push constant 0".to_owned());
                    buf.push("return".to_owned());
                }
            },
        }


        buf.join("\n")
    }
    fn expression_to_vm_code(&mut self,node: Expression) -> String{
        let mut buf: Vec<String> = vec![self.term_to_vm_code(node.intial_term)];

        for afterwards in node.afterwards{
            buf.push(self.term_to_vm_code(afterwards.1));
            // emit op:
            match afterwards.0.as_str() {
                "+" => {
                    buf.push("add".to_string());
                },
                "-" => {
                    buf.push("sub".to_string());
                },
                "*" => {
                    buf.push("call Math.multiply 2".to_string());
                },
                "/" => {
                    buf.push("call Math.divide 2".to_string());
                },
                "&" => {
                    buf.push("and".to_string());
                },
                "|" => {
                    buf.push("or".to_string());
                },
                ">" => {
                    buf.push("gt".to_string());
                },
                "<" => {
                    buf.push("lt".to_string());
                },
                "=" => {
                    buf.push("eq".to_string());
                },
                _ => unreachable!()
            }
        }

        buf.join("\n")
    }
    fn term_to_vm_code(&mut self,node: Term) -> String{
        let mut buf: Vec<String> = vec![];

        match node {
            Term::IntegerConstant(int) => buf.push(format!("push constant {}",int)),
            Term::StringConstant(_) => todo!(),
            Term::KeywordConstant(key) => {
                match key.as_str(){
                    "false" => {
                        buf.push("push constant 0".to_string());
                    },
                    "true" => {
                        buf.push("push constant 1".to_string());
                        buf.push("neg".to_string());
                    },
                    "null" => {
                        buf.push("push constant 0".to_string());
                    },
                    "this" => {
                        buf.push("push pointer 0".to_string());
                    },
                    _ => unreachable!()
                }
            },
            Term::VarName(var_name) => {
                let var_to_use;
                if let Some(var) = self.subroutine_table.table.get(&var_name){
                    var_to_use = var.clone();
                }else if let Some(var) = self.class_table.table.get(&var_name){
                    var_to_use = var.clone();
                } else {
                    panic!("var not found in class or subroutine to assign to with Term::VarName");
                }

                buf.push(format!("push {} {}",var_to_use.kind.to_string(),var_to_use.number))
            },
            Term::ArrayAccess { array_name, index } => todo!(),
            Term::SubroutineCall(call) => {
                buf.push(self.sub_call_to_vm_code(call));
            },
            Term::Expression(expr) => {
                buf.push(self.expression_to_vm_code(*expr));
            },
            Term::UnaryOp { op, term } => {
                buf.push(self.term_to_vm_code(*term));
                buf.push(match op.as_str(){
                    "-" => "neg".to_string(),
                    "~" => "not".to_string(),
                    _ => unreachable!()
                });
            },
        }


        buf.join("\n")
    }
    fn sub_call_to_vm_code(&mut self,call: SubroutineCall) -> String{
        let mut buf: Vec<String> = vec![];

        for argument in &call.arguments.list{
            buf.push(self.expression_to_vm_code(argument.clone()));
        }

        buf.push(format!("call {} {}",if call.obj_name.is_some(){
            format!("{}.{}",call.obj_name.unwrap(),call.subroutine_name)
        }else{
            call.subroutine_name
        },call.arguments.list.len()));


        buf.join("\n")
    }
}
