#[derive(Debug,Clone,PartialEq, Eq)]
pub enum TokenType{
    Keyword,
    Symbol,
    Identifier,
    IntegerConstant,
    StringConstant,
}
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Token{
    pub t_type: TokenType,
    pub string_repr: String,
}
// #[derive(Debug,Clone)]
// enum KeywordType{
//     Class,
//     Method,
//     Function,
//     Constructor,
//     Int,
//     Boolean,
//     Char,
//     Void,
//     Var,
//     Static,
//     Field,
//     Let,
//     Do,
//     If,
//     Else,
//     While,
//     Return,
//     True,
//     False,
//     Null,
//     This,
// }

impl Token{
    pub fn from_string(input_string: String) -> Token{
        let symbol_list = vec!["{" , "}" , "(" , ")" , "[" , "]" , "." , "," , ";" , "+" , "-" , "*" ,
        "/" , "&" , "|" , "<" , ">" , "=" , "~"];

        let keyword_list = vec![
            "class",
            "constructor",
            "function",
            "method",
            "field",
            "static",
            "var",
            "int",
            "char",
            "boolean",
            "void",
            "true",
            "false",
            "null",
            "this",
            "let",
            "do",
            "if",
            "else",
            "while",
            "return",
        ];

        if symbol_list.contains(&input_string.as_str()){
            Token{
                t_type: TokenType::Symbol,
                string_repr: input_string,
            }
        }
        else if input_string.chars().all(|ch| ch.is_ascii_digit()) && !input_string.is_empty(){
            let number = input_string.parse::<u64>().unwrap();
            if number > 32767{
                panic!("Integer Bigger than 32767 found!: {:?}",number);
            }
            Token{
                t_type: TokenType::IntegerConstant,
                string_repr: input_string,
            }
        }
        else if keyword_list.contains(&input_string.as_str()){
            Token{
                t_type: TokenType::Keyword,
                string_repr: input_string,
            }
        }
        else if input_string.starts_with('"') && input_string.ends_with('"'){
            Token{
                t_type: TokenType::StringConstant,
                string_repr: input_string.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string(),
            }
        }
        else {
            Token{
                t_type: TokenType::Identifier,
                string_repr: input_string,
            }
        }
    }
    pub fn to_xml_string(&self) -> String{
        match self.t_type {
            TokenType::Keyword => {
                format!("<keyword> {} </keyword>",self.string_repr)
            },
            TokenType::Symbol => {
                if self.string_repr == "<"{
                    "<symbol> &lt; </symbol>".to_string()
                }else if self.string_repr == ">"{
                    "<symbol> &gt; </symbol>".to_string()
                }else if self.string_repr == "&"{
                    "<symbol> &amp; </symbol>".to_string()
                }else {
                    format!("<symbol> {} </symbol>",self.string_repr)
                }
            },
            TokenType::Identifier => {
                format!("<identifier> {} </identifier>",self.string_repr)
            },
            TokenType::IntegerConstant => {
                format!("<integerConstant> {} </integerConstant>",self.string_repr)
            },
            TokenType::StringConstant => {
                format!("<stringConstant> {} </stringConstant>",self.string_repr)
            },
        }
    }
}

#[derive(Debug,Clone)]
pub struct Tokenizer{
    tokens: Vec<Token>,
    pos_idx: usize,
}

impl Tokenizer{
    pub fn from_string(input_string: String) -> Tokenizer{
        // Preproccess and remove Comments first

        let mut input_string = input_string.lines().filter(|line|{
            !line.starts_with("//")
        }).filter(|line| !(line.starts_with("//") || line.is_empty()))
        .map(|line| {
            if line.contains("//") {
                line.split_at(line.find("//").unwrap()).0.to_owned()
            } else {
                line.to_string()
            }
        }).collect::<Vec<String>>();

        let mut in_multiline_comment = false;

        for line in &mut input_string{
            if line.contains("/**"){
                if line.contains("*/"){
                    let after_multiline_end = line.split_at(line.find("*/").unwrap() + 2).1;
                    *line = after_multiline_end.to_string();
                    in_multiline_comment = false;
                }else{
                    let before_multiline = line.split_at(line.find("/**").unwrap()).0;
                    *line = before_multiline.to_string();
                    in_multiline_comment = true;
                }
            }
            if in_multiline_comment{
                if line.contains("*/"){
                    let after_multiline_end = line.split_at(line.find("*/").unwrap() + 2).1;
                    *line = after_multiline_end.to_string();
                    in_multiline_comment = false;
                }else {
                    *line = String::new();
                }
            }
        }
        let input_string = input_string.join(" ");

        


        let symbol_list = vec!['{' , '}' , '(' , ')' , '[' , ']' , '.' , ',' , ';' , '+' , '-' , '*' ,
        '/' , '&' , '|' , '<' , '>' , '=' , '~'];

        let mut new_tokenizer: Tokenizer = Tokenizer { tokens: vec![], pos_idx: 0 };

        let mut current_piece: String = "".to_owned();
        let mut string_mode = false;
        
        for character in input_string.chars(){
            if character == '"'{
                string_mode = !string_mode;
                current_piece.push(character);
                continue;
            }
            if string_mode{
                current_piece.push(character);
                continue;
            }
            if character.is_ascii_whitespace() {
                if current_piece.is_empty(){
                    continue;
                }
                let new_token = Token::from_string(current_piece.clone());
                current_piece = String::new();
                new_tokenizer.tokens.push(new_token);
                continue;
            }
            if character.is_ascii_alphabetic(){
                current_piece.push(character);
                continue;
            }
            if character == '_'{
                current_piece.push(character);
                continue;
            }
            if character.is_digit(10){
                current_piece.push(character);
                continue;
            }
            if symbol_list.contains(&character){
                if !current_piece.is_empty(){
                    let before_symbol = Token::from_string(current_piece.clone());
                    new_tokenizer.tokens.push(before_symbol);
                }
                
                current_piece = String::new();
                let symbol = Token::from_string(character.to_string());
                new_tokenizer.tokens.push(symbol);
                continue;
            }
            panic!("unknown char in Input Stream: {:?}",character);
        }
        // println!("From String: {:?}",new_tokenizer);

        new_tokenizer
    }
    pub fn to_xml_string(&self) -> String{
        let mut buffer: Vec<String> = vec!["<tokens>".to_string()];

        for token in &self.tokens{
            buffer.push(token.to_xml_string());
        }

        buffer.push("</tokens>".to_string());
        buffer.join("\n")
    }
}

impl Tokenizer{
    pub fn current(&self) -> Token{
        self.tokens[self.pos_idx].clone()
    }
    pub fn next(&self) -> Token{
        self.tokens[self.pos_idx+1].clone()
    }
    pub fn advance(&mut self) -> Token{
        let temp = self.current();
        self.pos_idx += 1;
        temp
    }
    pub fn expect(&mut self,expected_type: TokenType) -> Token{
        if self.current().t_type == expected_type{
            self.advance()
        }else{
            panic!("Expected Type: {:?},found: {:?}",expected_type,self.current());
        }
    }
    pub fn expect_and_string(&mut self,expected_type: TokenType,expected_string: &str) -> Token{
        if self.current().t_type == expected_type && self.current().string_repr == expected_string{
            self.advance()
        }else{
            panic!("Expected Type: {:?} and String: {:?},found: {:?}",expected_type,expected_string,self.current());
        }
    }
    pub fn still_left(&self) -> bool{
        self.pos_idx < self.tokens.len()
    }
}