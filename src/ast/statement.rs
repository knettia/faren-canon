use dyn_clone::DynClone;
use std::{any::Any, collections::VecDeque};

use crate::{
	data::vtype::*,
	parser::token::Token
};

use super::expression::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementType
{
	FunctionDefine,
	FunctionDeclare,
	FunctionReturn,

	Expression,

	Compound,
	Declare,
	Assign,
	Print
}

pub trait StatementTrait: DynClone
{
	fn stype(&self) -> StatementType;
	fn unparse(&self) -> VecDeque<Token>;
	fn as_any(&self) -> &dyn Any; // for downcasting
}

dyn_clone::clone_trait_object!(StatementTrait);

#[derive(Clone)]
pub struct FunctionDefineStatement
{
	tokens: VecDeque<Token>,

	signature: FunctionSignature,
	body: CompoundStatement
}

impl StatementTrait for FunctionDefineStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::FunctionDefine
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl FunctionDefineStatement
{
	pub fn new(tokens: VecDeque<Token>, signature: FunctionSignature, body: CompoundStatement) -> Self
	{
		Self { tokens, signature, body }
	}

	pub fn signature(&self) -> FunctionSignature
	{
		self.signature.clone()
	}

	pub fn body(&self) -> CompoundStatement
	{
		self.body.clone()
	}
}

#[derive(Clone)]
pub struct FunctionDeclareStatement
{
	tokens: VecDeque<Token>,

	signature: FunctionSignature
}

impl StatementTrait for FunctionDeclareStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::FunctionDeclare
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl FunctionDeclareStatement
{
	pub fn new(tokens: VecDeque<Token>, signature: FunctionSignature) -> Self
	{
		Self { tokens, signature }
	}

	pub fn signature(&self) -> FunctionSignature
	{
		self.signature.clone()
	}
}

#[derive(Clone)]
pub struct FunctionReturnStatement
{
	tokens: VecDeque<Token>,

	expression: Option<Expression>
}

impl StatementTrait for FunctionReturnStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::FunctionReturn
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl FunctionReturnStatement
{
	pub fn new(tokens: VecDeque<Token>, expression: Option<Expression>) -> Self
	{
		Self { tokens, expression }
	}

	pub fn expression(&self) -> Option<Expression>
	{
		self.expression.clone()
	}
}

#[derive(Clone)]
pub struct ExpressionStatement
{
	tokens: VecDeque<Token>,

	expression: Expression
}

impl StatementTrait for ExpressionStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::Expression
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl ExpressionStatement
{
	pub fn new(tokens: VecDeque<Token>, expression: Expression) -> Self
	{
		Self { tokens, expression }
	}

	pub fn expression(&self) -> Expression
	{
		self.expression.clone()
	}
}

#[derive(Clone)]
pub struct CompoundStatement
{
	tokens: VecDeque<Token>,

	statements: VecDeque<Statement>
}

impl StatementTrait for CompoundStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::Compound
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl CompoundStatement
{
	pub fn new(tokens: VecDeque<Token>, statements: VecDeque<Statement>) -> Self
	{
		Self { tokens, statements }
	}

	pub fn statements(&self) -> VecDeque<Statement>
	{
		self.statements.clone()
	}
}

#[derive(Clone)]
pub struct DeclareStatement
{
	tokens: VecDeque<Token>,

	vtype: VType,
	identifier: u16,
	expression: Expression
}

impl StatementTrait for DeclareStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::Declare
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl DeclareStatement
{
	pub fn new(tokens: VecDeque<Token>, vtype: VType, identifier: u16, expression: Expression) -> Self
	{
		Self { tokens, vtype, identifier, expression }
	}

	pub fn vtype(&self) -> VType
	{
		self.vtype.clone()
	}

	pub fn identifier(&self) -> u16
	{
		self.identifier
	}

	pub fn expression(&self) -> Expression
	{
		self.expression.clone()
	}
}

#[derive(Clone)]
pub struct AssignStatement
{
	tokens: VecDeque<Token>,

	identifier: u16,
	expression: Expression
}

impl StatementTrait for AssignStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::Assign
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl AssignStatement
{
	pub fn new(tokens: VecDeque<Token>, identifier: u16, expression: Expression) -> Self
	{
		Self { tokens, identifier, expression }
	}

	pub fn identifier(&self) -> u16
	{
		self.identifier
	}

	pub fn expression(&self) -> Expression
	{
		self.expression.clone()
	}
}

#[derive(Clone)]
pub struct PrintStatement
{
	tokens: VecDeque<Token>,
	
	expression: Expression
}

impl StatementTrait for PrintStatement
{
	fn stype(&self) -> StatementType
	{
		StatementType::Print
	}

	fn unparse(&self) -> VecDeque<Token>
	{
		self.tokens.clone()
	}

	fn as_any(&self) -> &dyn Any
	{
		self
	}
}

impl PrintStatement
{
	pub fn new(tokens: VecDeque<Token>, expression: Expression) -> Self
	{
		Self { tokens, expression }
	}

	pub fn expression(&self) -> Expression
	{
		self.expression.clone()
	}
}

pub type StatementBox = Box<dyn StatementTrait>;

#[derive(Clone)]
pub struct Statement
{
	statement: StatementBox
}

impl Statement
{
	// Token functions:
	pub fn stype(&self) -> StatementType
	{
		self.statement.stype()
	}

	// New functions:
	pub fn new(statement: StatementBox) -> Self
	{
		Self { statement }
	}

	pub fn new_function_define(tokens: VecDeque<Token>, signature: FunctionSignature, body: CompoundStatement) -> Self
	{
		Self::new(Box::new(FunctionDefineStatement::new(tokens, signature, body)))
	}

	pub fn new_function_declare(tokens: VecDeque<Token>, signature: FunctionSignature) -> Self
	{
		Self::new(Box::new(FunctionDeclareStatement::new(tokens, signature)))
	}

	pub fn new_function_return(tokens: VecDeque<Token>, expression: Option<Expression>) -> Self
	{
		Self::new(Box::new(FunctionReturnStatement::new(tokens, expression)))
	}

	pub fn new_expression(tokens: VecDeque<Token>, expression: Expression) -> Self
	{
		Self::new(Box::new(ExpressionStatement::new(tokens, expression)))
	}

	pub fn new_compound(tokens: VecDeque<Token>, statements: VecDeque<Statement>) -> Self
	{
		Self::new(Box::new(CompoundStatement::new(tokens, statements)))
	}
	
	pub fn new_declare(tokens: VecDeque<Token>, vtype: VType, identifier: u16, expression: Expression) -> Self
	{
		Self::new(Box::new(DeclareStatement::new(tokens, vtype, identifier, expression)))
	}

	pub fn new_assign(tokens: VecDeque<Token>, identifier: u16, expression: Expression) -> Self
	{
		Self::new(Box::new(AssignStatement::new(tokens, identifier, expression)))
	}

	pub fn new_print(tokens: VecDeque<Token>, expression: Expression) -> Self
	{
		Self::new(Box::new(PrintStatement::new(tokens, expression)))
	}

	// As function:
	pub fn as_statement<T: 'static>(&self) -> Option<&T>
	{
		self.statement.as_any().downcast_ref::<T>()
	}
}
