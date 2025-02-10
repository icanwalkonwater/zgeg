use super::Ast;

pub struct File {
    pub items: Vec<Ast<Item>>,
}

pub enum Item {
    Function(Ast<FunctionItem>),
}

pub struct FunctionItem {
    pub name: Ast<Identifier>,
    pub return_type: Option<Ast<Type>>,
    pub body: Ast<Body>,
}

pub struct Body {
    pub statements: Vec<Ast<Statement>>,
}

pub enum Statement {
    Let(Ast<LetStatement>),
    Assignement(Ast<AssignementStatement>),
    Return(Option<Ast<Expression>>),
    Expression(Ast<Expression>),
}

pub struct LetStatement {
    pub name: Ast<Identifier>,
    pub type_: Option<Ast<Type>>,
    pub initializer: Option<Ast<Expression>>,
}

pub struct AssignementStatement {
    pub place: Ast<Identifier>,
    pub value: Ast<Expression>,
}

pub enum Expression {
    Binary(Ast<BinaryExpression>),
    Unary(Ast<UnaryExpression>),
    FunctionCall(Ast<FunctionCall>),
    Identifier(Ast<Identifier>),
    Literal(Ast<Literal>),
}

pub struct BinaryExpression {
    pub left: Ast<Expression>,
    pub op: Ast<BinaryOp>,
    pub right: Ast<Expression>,
}

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

pub struct UnaryExpression {
    pub op: Ast<UnaryOp>,
    pub value: Ast<Expression>,
}

pub enum UnaryOp {
    Hyphen,
    Bang,
}

pub struct FunctionCall {
    pub function_name: Ast<Identifier>,
    pub parameters: Vec<Ast<Expression>>,
}

enum Literal {
    String(Ast<String>),
    Number(Ast<String>),
}

struct Type(pub Ast<Identifier>);

struct Identifier(pub Ast<String>);
