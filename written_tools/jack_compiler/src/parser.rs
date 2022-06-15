#![allow(non_snake_case)]

use crate::tokenizer::{Token, Tokenizer, TokenType::*};

// Not needed Types:
// type
// className
// subroutineName
// variableName
// statement
// subroutineCall
enum ParseNodeType{
    Class,
    ClassVarDec,
    SubroutineDec,
    ParameterList,
    SubroutineBody,
    VarDec,
    Statements,
    Let,
    If,
    While,
    Do,
    Return,
    Expression,
    Term,
    ExpressionList,
}
enum TokenOrParseNode{
    Token(Token),
    ParseNode(ParseNode),
}


pub struct ParseNode{
    node_type: ParseNodeType,
    body: Vec<TokenOrParseNode>,
}


pub struct Parser{
    tokenizer: Tokenizer,
    node_list: Vec<ParseNode>,
}

impl Parser{
    pub fn from_tokenizer(tokenizer: Tokenizer) -> Parser{
        Self { tokenizer, node_list: vec![] }
    }
    pub fn compileClass(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Class,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Keyword,"class")));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(crate::tokenizer::TokenType::Identifier)));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,"{")));

        // classVarDec*
        while self.tokenizer.current().t_type == Keyword && (self.tokenizer.current().string_repr == "static" || self.tokenizer.current().string_repr == "field"){
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileClassVarDec()));
        }

        // subroutineDec*
        while self.tokenizer.current().t_type == Keyword && (self.tokenizer.current().string_repr == "constructor" || self.tokenizer.current().string_repr == "function" || self.tokenizer.current().string_repr == "method"){
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileSubroutineDec()));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"}")));

        node_temp
    }
    fn compileClassVarDec(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::ClassVarDec,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword && !(type_of_dec.string_repr == "int" || type_of_dec.string_repr == "char" || type_of_dec.string_repr == "boolean"){
            panic!("Expected int char or boolean as type if it is a keyword!");
        }else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier{
            node_temp.body.push(TokenOrParseNode::Token(type_of_dec));
        }else{ 
            panic!("Expected int,char,boolean or Identifier as type in compileClassVarDec!");
        }
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

        while self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ","{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,";")));

        node_temp
    }
    fn compileSubroutineDec(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::SubroutineDec,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword && !(type_of_dec.string_repr == "int" || type_of_dec.string_repr == "char" || type_of_dec.string_repr == "boolean" ||  type_of_dec.string_repr == "void"){
            panic!("Expected int char void or boolean as type if it is a keyword!");
        }else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier{
            node_temp.body.push(TokenOrParseNode::Token(type_of_dec));
        }else{ 
            panic!("Expected int,char,boolean,void or Identifier as type in compileSubroutineDec!");
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,"(")));

        if !(self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ")"){
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileParameterList()));
        }else {
            node_temp.body.push(TokenOrParseNode::ParseNode(ParseNode{
                node_type: ParseNodeType::ParameterList,
                body: vec![],
            }));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,")")));

        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileSubroutineBody()));

        node_temp
    }
    fn compileParameterList(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::ParameterList,
            body: vec![],
        };

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword && !(type_of_dec.string_repr == "int" || type_of_dec.string_repr == "char" || type_of_dec.string_repr == "boolean"){
            panic!("Expected int char or boolean as type if it is a keyword!");
        }else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier{
            node_temp.body.push(TokenOrParseNode::Token(type_of_dec));
        }else{ 
            panic!("Expected int,char,boolean or Identifier as type in compileParameterList!");
        }
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

        while self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ","{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            let type_of_dec = self.tokenizer.advance();
            if type_of_dec.t_type == Keyword && !(type_of_dec.string_repr == "int" || type_of_dec.string_repr == "char" || type_of_dec.string_repr == "boolean"){
                panic!("Expected int char or boolean as type if it is a keyword!");
            }else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier{
                node_temp.body.push(TokenOrParseNode::Token(type_of_dec));
            }else{ 
                panic!("Expected int,char,boolean or Identifier as type in compileParameterList!");
            }
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));
        }
        
        node_temp
    }
    fn compileSubroutineBody(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::SubroutineBody,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,"{")));

        // varDec*
        while self.tokenizer.current().t_type == Keyword && self.tokenizer.current().string_repr == "var"{
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileVarDec()));
        }

        // statements
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileStatements()));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,"}")));
        node_temp
    }
    fn compileVarDec(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::VarDec,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        let type_of_dec = self.tokenizer.advance();
        if type_of_dec.t_type == Keyword && !(type_of_dec.string_repr == "int" || type_of_dec.string_repr == "char" || type_of_dec.string_repr == "boolean"){
            panic!("Expected int char or boolean as type if it is a keyword!");
        }else if type_of_dec.t_type == Keyword || type_of_dec.t_type == Identifier{
            node_temp.body.push(TokenOrParseNode::Token(type_of_dec));
        }else{ 
            panic!("Expected int,char,boolean or Identifier as type in compileClassVarDec!");
        }
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

        while self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ","{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(crate::tokenizer::TokenType::Symbol,";")));

        node_temp
    }
    fn compileStatements(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Statements,
            body: vec![],
        };
        // statement*
        // either let,if,while,do,return
        while self.tokenizer.current().t_type == Keyword && (self.tokenizer.current().string_repr == "let" || self.tokenizer.current().string_repr == "if" || self.tokenizer.current().string_repr == "while" || self.tokenizer.current().string_repr == "do" || self.tokenizer.current().string_repr == "return"){
            match self.tokenizer.current().string_repr.as_str() {
                "let" => {
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileLet()));
                },
                "if" => {
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileIf()));
                },
                "while" => {
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileWhile()));
                },
                "do" => {
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileDo()));
                },
                "return" => {
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileReturn()));
                },
                _ => unreachable!()
            }
        }

        node_temp
    }
    fn compileLet(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Let,
            body: vec![],
        };

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

        if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "["{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"]")));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"=")));

        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,";")));
        node_temp
    }
    fn compileIf(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::If,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"(")));
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"{")));
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileStatements()));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"}")));

        if self.tokenizer.current().t_type == Keyword && self.tokenizer.current().string_repr == "else"{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"{")));
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileStatements()));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"}")));
        }

        node_temp
    }
    fn compileWhile(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::While,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"(")));
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"{")));
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileStatements()));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"}")));

        node_temp
    }
    fn compileDo(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Do,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));
        if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "."{
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Symbol)));
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));
        }
        
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"(")));
        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpressionList()));
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,";")));
        node_temp
    }
    fn compileReturn(&mut self) -> ParseNode{
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Return,
            body: vec![],
        };
        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Keyword)));

        if !(self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ";"){
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
        }

        node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,";")));
        node_temp
    }
    fn compileExpression(&mut self) -> ParseNode {
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Expression,
            body: vec![],
        };

        node_temp.body.push(TokenOrParseNode::ParseNode(self.compileTerm()));
        while self.tokenizer.current().t_type == Symbol && (self.tokenizer.current().string_repr == "+" || self.tokenizer.current().string_repr == "-" || self.tokenizer.current().string_repr == "*" || self.tokenizer.current().string_repr == "/" || self.tokenizer.current().string_repr == "&" || self.tokenizer.current().string_repr == "|" || self.tokenizer.current().string_repr == "<" || self.tokenizer.current().string_repr == ">" || self.tokenizer.current().string_repr == "="){
            node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileTerm()));
        }

        node_temp
    }
    fn compileTerm(&mut self) -> ParseNode {
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::Term,
            body: vec![],
        };
        match self.tokenizer.current().t_type {
            Keyword => {
                // either true false null or this
                if self.tokenizer.current().string_repr == "true" ||  self.tokenizer.current().string_repr == "false" || self.tokenizer.current().string_repr == "null" || self.tokenizer.current().string_repr == "this"{
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
                } else{
                    panic!("keyword in Term other than true false or null or this arent possible! found: {:?}",self.tokenizer.current());
                }
            },
            Symbol => {
                // either expression in paratheses or unaryOp
                if self.tokenizer.current().string_repr == "-" ||  self.tokenizer.current().string_repr == "~"{
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileTerm()));
                }else if self.tokenizer.current().string_repr == "(" {
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));
                } else {
                    panic!("Symbol in Term other than ~,- or (  arent possible! found: {:?}",self.tokenizer.current());
                }
            },
            Identifier => {
                node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
                if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "["{
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"[")));
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"]")));

                }else if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "("{
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"(")));
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpressionList()));
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));
                } else if self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == "."{
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Symbol)));
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect(Identifier)));

                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,"(")));
                    node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpressionList()));
                    node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.expect_and_string(Symbol,")")));
                }
            },
            IntegerConstant => {
                node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            },
            StringConstant => {
                node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
            },
        }

        node_temp
    }
    fn compileExpressionList(&mut self) -> ParseNode {
        let mut node_temp = ParseNode{
            node_type: ParseNodeType::ExpressionList,
            body: vec![],
        };

        // only if there is a expression list
        if self.tokenizer.current().string_repr != ")"{
            node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));

            while self.tokenizer.current().t_type == Symbol && self.tokenizer.current().string_repr == ","{
                node_temp.body.push(TokenOrParseNode::Token(self.tokenizer.advance()));
                node_temp.body.push(TokenOrParseNode::ParseNode(self.compileExpression()));
            }
        }

        node_temp
    }
}

impl ParseNode{
    pub fn to_xml_string(&self) -> String{
        let mut buf: Vec<String>= vec![];

        let xml_string = match self.node_type {
            ParseNodeType::Class => "class",
            ParseNodeType::ClassVarDec => "classVarDec",
            ParseNodeType::SubroutineDec => "subroutineDec",
            ParseNodeType::ParameterList => "parameterList",
            ParseNodeType::SubroutineBody => "subroutineBody",
            ParseNodeType::VarDec => "varDec",
            ParseNodeType::Statements => "statements",
            ParseNodeType::Let => "letStatement",
            ParseNodeType::If => "ifStatement",
            ParseNodeType::While => "whileStatement",
            ParseNodeType::Do => "doStatement",
            ParseNodeType::Return => "returnStatement",
            ParseNodeType::Expression => "expression",
            ParseNodeType::Term => "term",
            ParseNodeType::ExpressionList => "expressionList",
        };
        
        buf.push(format!("<{}>",xml_string));

        for child in &self.body{
            match child{
                TokenOrParseNode::Token(token) => {
                    buf.push(token.to_xml_string());
                },
                TokenOrParseNode::ParseNode(parse_node) => {
                    buf.push(parse_node.to_xml_string());
                },
            }
        }

        buf.push(format!("</{}>",xml_string));

        buf.join("\n")
    }
}