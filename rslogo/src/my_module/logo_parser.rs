use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, char,line_ending,  multispace0, multispace1, digit1},
    combinator::{map, recognize, rest},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};


#[derive(Debug, PartialEq)]
pub enum ProgramProcedure {
    XCOR,
    YCOR,
    HEADING,
    COLOR,
}

#[derive(Debug, PartialEq)]
pub enum Expr { //Expressions are things that can produce a value
    Variable(String),
    Literal(String),
    ProcedureCall(String, Vec<Expr>),
    ProgramProcedure(ProgramProcedure),
    ADD(Box<Expr>, Box<Expr>),     
    SUB(Box<Expr>, Box<Expr>),
    MUL(Box<Expr>, Box<Expr>),
    DIV(Box<Expr>, Box<Expr>),
    EQ(Box<Expr>, Box<Expr>),
    LT(Box<Expr>, Box<Expr>),
    GT(Box<Expr>, Box<Expr>),
    NE(Box<Expr>, Box<Expr>),
    AND(Box<Expr>, Box<Expr>),
    OR(Box<Expr>, Box<Expr>)
}




#[derive(Debug, PartialEq)]
pub enum Statement { 
    MakeVariable(Expr, Expr),
    AddAssign(Expr, Expr),
    PenUp,
    PenDown,
    Left(Expr),
    Right(Expr),
    Forward(Expr),
    Back(Expr),
    Turn(Expr),
    SetPenColor(Expr),
    SetHeading(Expr),
    SetX(Expr),
    SetY(Expr),
    ProceduresDefinition(String, Vec<String>, Vec<Statement>), 
    Comment,
    IF(Expr, Vec<Statement>),
    WHILE(Expr, Vec<Statement>)
}



#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub fn parse_comment(input: &str) -> IResult<&str, ()> {
    map(
        tuple((
            tag("//"),
            take_until("\n"),
            alt((line_ending, rest)),
        )),
        |_| (),
    )(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(
            many1(alt((alphanumeric1, tag("_"))))
        ),
        String::from,
    )(input)
}

pub fn parse_condition_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        // 解析 AND 条件
        map(
            tuple((
                tag("AND"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::AND(Box::new(expr1), Box::new(expr2))
        ),
        // 解析 OR 条件
        map(
            tuple((
                tag("OR"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::OR(Box::new(expr1), Box::new(expr2))
        ),
        // 解析 EQ 条件
        map(
            tuple((
                tag("EQ"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::EQ(Box::new(expr1), Box::new(expr2))
        ),
        // 解析 LT 条件
        map(
            tuple((
                tag("LT"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::LT(Box::new(expr1), Box::new(expr2))
        ),
        // 解析 GT 条件
        map(
            tuple((
                tag("GT"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::GT(Box::new(expr1), Box::new(expr2))
        ),
        // 解析 NE 条件
        map(
            tuple((
                tag("NE"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::NE(Box::new(expr1), Box::new(expr2))
        ),
    ))(input)
}

pub fn parse_arithmetic_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((
                tag("+"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::ADD(Box::new(expr1), Box::new(expr2))
        ),
        map(
            tuple((
                tag("-"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::SUB(Box::new(expr1), Box::new(expr2))
        ),
        map(
            tuple((
                tag("*"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::MUL(Box::new(expr1), Box::new(expr2))
        ),
        map(
            tuple((
                tag("/"),
                multispace1,
                parse_expr,
                multispace1,
                parse_expr
            )),
            |(_, _, expr1, _, expr2)| Expr::DIV(Box::new(expr1), Box::new(expr2))
        ),
    ))(input)
}

pub fn parse_if(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("IF"),
            multispace1,
            parse_expr,
            multispace1,
            tag("["),
            multispace0,
            many0(delimited(  
                multispace0,
                parse_statement,  
                multispace0
            )),
            tag("]")
        )),
        |(_, _, condition, _, _, _, statements, _)| {
            Statement::IF(condition, statements)
        }
    )(input)
}

pub fn parse_while(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("WHILE"),
            multispace1,
            parse_expr,
            multispace1,
            tag("["),
            multispace0,
            many0(delimited(
                multispace0,
                parse_statement,
                multispace0
            )),
            tag("]")
        )),
        |(_, _, condition, _, _, _, statements, _)| {
            Statement::WHILE(condition, statements)
        }
    )(input)
}

pub fn parse_literal(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            char('"'),
            recognize(
                pair(
                    alt((alpha1, tag("_"), digit1, tag("-"))),
                    many0(alt((alphanumeric1, tag("_"), tag("-"))))
                )
            )
        ),
        |s: &str| Expr::Literal(s.to_string())
    )(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(char(':'), parse_identifier),
        Expr::Variable,
    )(input)
}

pub fn parse_procedure_call(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            parse_identifier,
            many0(preceded(multispace1, parse_expr)),
        )),
        |(name, args)| Expr::ProcedureCall(name, args),
    )(input)
}

pub fn parse_program_procedure(input: &str) -> IResult<&str, Expr> {
    let (input, procedure) = alt((
        map(tag("XCOR"), |_| ProgramProcedure::XCOR),
        map(tag("YCOR"), |_| ProgramProcedure::YCOR),
        map(tag("HEADING"), |_| ProgramProcedure::HEADING),
        map(tag("COLOR"), |_| ProgramProcedure::COLOR),
    ))(input)?;
    
    Ok((input, Expr::ProgramProcedure(procedure)))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_arithmetic_expr,
        parse_condition_expr,
        parse_literal,
        parse_variable,
        parse_program_procedure,
        parse_procedure_call,
        
    ))(input)
}

pub fn parse_make(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("MAKE"),
            multispace1,
            parse_expr, 
            multispace1,
            parse_expr,
        )),
        |(_, _, name, _, value)| Statement::MakeVariable(name, value),
    )(input)
}

pub fn parse_addassign(input: &str) -> IResult<&str, Statement>{
    map(
        tuple((
            tag("ADDASSIGN"),
            multispace1,
            parse_expr, 
            multispace1,
            parse_expr,
        )),
        |(_, _, name, _, value)| Statement::AddAssign(name, value),
    )(input)
}

pub fn parse_penup(input: &str) -> IResult<&str, Statement> {
    map(tag("PENUP"), |_| Statement::PenUp)(input)
}

pub fn parse_pendown(input: &str) -> IResult<&str, Statement> {
    map(tag("PENDOWN"), |_| Statement::PenDown)(input)
}

pub fn parse_forward(input: &str) -> IResult<&str, Statement> {
    map(
        preceded(pair(tag("FORWARD"), multispace1), parse_expr),
        Statement::Forward,
    )(input)
}

pub fn parse_back(input: &str) -> IResult<&str, Statement>{
    map(
        preceded(pair(tag("BACK"), multispace1), parse_expr),
        Statement::Back,
    )(input)
}

pub fn parse_left(input: &str) -> IResult<&str, Statement>{
    map(
        preceded(pair(tag("LEFT"), multispace1), parse_expr),
        Statement::Left,
    )(input)
}

pub fn parse_right(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("RIGHT"), multispace1), parse_expr),
        Statement::Right,
    )(input)
}

pub fn parse_turn(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("TURN"), multispace1), parse_expr),
        Statement::Turn,
    )(input)
}

pub fn parse_setpencolor(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("SETPENCOLOR"), multispace1), parse_expr),
        Statement::SetPenColor,
    )(input)
}

pub fn parse_setheading(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("SETHEADING"), multispace1), parse_expr),
        Statement::SetHeading,
    )(input)
}

pub fn parse_setx(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("SETX"), multispace1), parse_expr),
        Statement::SetX,
    )(input)
}

pub fn parse_sety(input: &str) -> IResult<&str,Statement>{
    map(
        preceded(pair(tag("SETY"), multispace1), parse_expr),
        Statement::SetY,
    )(input)
}

pub fn parse_procedure_definition(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("TO"),
            multispace1,
            parse_identifier,
            many0(preceded(multispace1, preceded(char(':'), parse_identifier))),
            multispace0,
            many1(delimited(multispace0, parse_statement, multispace0)),
            tag("END"),
        )),
        |(_, _, name, params, _, body, _)| {
            Statement::ProceduresDefinition(name, params, body)
        },
    )(input)
}



pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_make,
        parse_addassign,
        parse_penup,
        parse_pendown,
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
        parse_turn,
        parse_setpencolor,
        parse_setheading,
        parse_setx,
        parse_sety,
        parse_procedure_definition,
        parse_if,
        parse_while,
        map(parse_comment, |_| Statement::Comment)
    ))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    println!("Parsing input: {:?}", input);  
    let result = map(
        many0(delimited(
            multispace0,
            alt((
                map(parse_statement, Some),
                map(parse_comment, |_| None)
            )),
            multispace0
        )),
        |statements| {
            let filtered: Vec<_> = statements.into_iter().filter_map(|s| s).collect();
            println!("Parsed statements: {:#?}", filtered);  
            Program { 
                statements: filtered 
            }
        },
    )(input);
    println!("Parse result: {:#?}", result);  
    result
}
