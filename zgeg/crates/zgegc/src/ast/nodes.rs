use super::Ast;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Ast<Item>>,
}

#[derive(Debug)]
pub enum Item {
    Function(Ast<FunctionItem>),
}

#[derive(Debug)]
pub struct FunctionItem {
    pub name: Ast<Identifier>,
    pub return_type: Option<Ast<Type>>,
    pub body: Ast<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Ast<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Ast<LetStatement>),
    Assignement(Ast<AssignementStatement>),
    Return(Ast<ReturnStatement>),
    Expression(Ast<Expression>),
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Ast<Identifier>,
    pub type_: Option<Ast<Type>>,
    pub initializer: Option<Ast<Expression>>,
}

#[derive(Debug)]
pub struct AssignementStatement {
    pub place: Ast<Identifier>,
    pub value: Ast<Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub return_value: Option<Ast<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    Binary(Ast<BinaryExpression>),
    Unary(Ast<UnaryExpression>),
    FunctionCall(Ast<FunctionCall>),
    Identifier(Ast<Identifier>),
    Literal(Ast<Literal>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Ast<Expression>,
    pub op: Ast<BinaryOp>,
    pub right: Ast<Expression>,
}

#[derive(Debug)]
pub enum BinaryOp {
    Star,
    Slash,
    Plus,
    Hyphen,
    EqualEqual,
    BangEqual,
    AngleLeft,
    AngleRight,
    AngleLeftEqual,
    AngleRightEqual,
    AmpersandAmpersand,
    VerticalBarVerticalBar,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub op: Ast<UnaryOp>,
    pub value: Ast<Expression>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Hyphen,
    Bang,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function_name: Ast<Identifier>,
    pub parameters: Vec<Ast<Expression>>,
}

#[derive(Debug)]
pub enum Literal {
    String(Ast<String>),
    Number(Ast<String>),
}

#[derive(Debug)]
pub struct Type(pub Ast<Identifier>);

#[derive(Debug)]
pub struct Identifier(pub Ast<String>);
