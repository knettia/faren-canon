use dyn_clone::DynClone;
use std::any::Any;
use std::collections::VecDeque;

use super::literal::Literal;

use crate::{
	data::{
		ops::*,
		vtype::*
	},
	parser::token::Token
};

pub enum ExpressionType
{
	Literal,
	FunctionCall,
	Variable,
	Arithmetic,
	Comparison,
	Boolean
} 

pub trait ExpressionTrait: DynClone
{
	fn vtype(&self) -> VType;
	fn etype(&self) -> ExpressionType;

	fn unparse(&self) -> VecDeque<Token>;

	fn as_any(&self) -> &dyn Any; // for downcasting
}

dyn_clone::clone_trait_object!(ExpressionTrait);

#[derive(Clone)]
pub struct LiteralExpression
{
	tokens: VecDeque<Token>,
	literal: Literal
}

impl ExpressionTrait for LiteralExpression
{
	fn vtype(&self) -> VType
	{
		self.literal.vtype().clone()
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::Literal
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

impl LiteralExpression
{
	fn new(tokens: VecDeque<Token>, literal: Literal) -> Self
	{
		Self { tokens, literal }
	}

	fn literal(&self) -> &Literal
	{
		&self.literal
	}
}

#[derive(Clone)]
pub struct FunctionCallExpression
{
	tokens: VecDeque<Token>,
	vtype: VType,
	name: String,
	passed_arguments: VecDeque<Expression>
}


impl ExpressionTrait for FunctionCallExpression
{
	fn vtype(&self) -> VType
	{
		self.vtype.clone()
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::FunctionCall
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

impl FunctionCallExpression
{
	fn new(tokens: VecDeque<Token>, vtype: VType, name: String, passed_arguments: VecDeque<Expression>) -> Self
	{
		Self { tokens, vtype, name, passed_arguments }
	}

	pub fn name(&self) -> String
	{
		self.name.clone()
	}

	pub fn passed_arguments(&self) -> VecDeque<Expression>
	{
		self.passed_arguments.clone()
	}
}

#[derive(Clone)]
pub struct VariableExpression
{
	tokens: VecDeque<Token>,
	vtype: VType,
	identifier: u16
}

impl ExpressionTrait for VariableExpression
{
	fn vtype(&self) -> VType
	{
		self.vtype.clone()
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::Variable
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

impl VariableExpression
{
	fn new(tokens: VecDeque<Token>, vtype: VType, identifier: u16) -> Self
	{
		Self { tokens, vtype, identifier }
	}

	pub fn vtype(&self) -> VType
	{
		self.vtype.clone()
	}

	pub fn identifier(&self) -> u16
	{
		self.identifier
	}
}

#[derive(Clone)]
pub struct ArithmeticExpression
{
	tokens: VecDeque<Token>,

	vtype: VType,
	op: ArithmeticOperation,

	left: Expression,
	right: Expression
}

impl ExpressionTrait for ArithmeticExpression
{
	fn vtype(&self) -> VType
	{
		self.vtype.clone()
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::Arithmetic
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

impl ArithmeticExpression
{
	pub fn new(tokens: VecDeque<Token>, vtype: VType, op: ArithmeticOperation, left: Expression, right: Expression) -> Self
	{
		Self { tokens, vtype, op, left, right }
	}

	pub fn op(&self) -> ArithmeticOperation
	{
		self.op.clone()
	}

	pub fn left(&self) -> Expression
	{
		self.left.clone()
	}

	pub fn right(&self) -> Expression
	{
		self.right.clone()
	}
}

#[derive(Clone)]
pub struct ComparisonExpression
{
	tokens: VecDeque<Token>,

	op: ComparisonOperation,

	left: Expression,
	right: Expression
}

impl ExpressionTrait for ComparisonExpression
{
	fn vtype(&self) -> VType
	{
		VType::Boolean
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::Comparison
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

impl ComparisonExpression
{
	pub fn new(tokens: VecDeque<Token>, op: ComparisonOperation, left: Expression, right: Expression) -> Self
	{
		Self { tokens, op, left, right }
	}

	pub fn op(&self) -> ComparisonOperation
	{
		self.op.clone()
	}

	pub fn left(&self) -> Expression
	{
		self.left.clone()
	}

	pub fn right(&self) -> Expression
	{
		self.right.clone()
	}
}

#[derive(Clone)]
pub struct BooleanExpression
{
	tokens: VecDeque<Token>,

	op: BooleanOperation,

	left: Expression,
	right: Expression
}

impl ExpressionTrait for BooleanExpression
{
	fn vtype(&self) -> VType
	{
		VType::Boolean
	}

	fn etype(&self) -> ExpressionType
	{
		ExpressionType::Boolean
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

impl BooleanExpression
{
	pub fn new(tokens: VecDeque<Token>, op: BooleanOperation, left: Expression, right: Expression) -> Self
	{
		Self { tokens, op, left, right }
	}

	pub fn op(&self) -> BooleanOperation
	{
		self.op.clone()
	}

	pub fn left(&self) -> Expression
	{
		self.left.clone()
	}

	pub fn right(&self) -> Expression
	{
		self.right.clone()
	}
}

pub type ExpressionBox = Box<dyn ExpressionTrait>;

#[derive(Clone)]
pub struct Expression
{
	expression: ExpressionBox
}

impl Expression
{
	// Token functions:
	pub fn virtual_type(&self) -> VType
	{
		self.expression.vtype()
	}

	pub fn expression_type(&self) -> ExpressionType
	{
		self.expression.etype()
	}

	// New functions:
	pub fn new(expression: ExpressionBox) -> Self
	{
		Self { expression }
	}

	pub fn new_literal(tokens: VecDeque<Token>, literal: Literal) -> Self
	{
		Self::new(Box::new(LiteralExpression::new(tokens, literal)))
	}

	pub fn new_function_call(tokens: VecDeque<Token>, vtype: VType, name: String, passed_arguments: VecDeque<Expression>) -> Self
	{
		Self::new(Box::new(FunctionCallExpression::new(tokens, vtype, name, passed_arguments)))
	}
	
	pub fn new_variable(tokens: VecDeque<Token>, vtype: VType, identifier: u16) -> Self
	{
		Self::new(Box::new(VariableExpression::new(tokens, vtype, identifier)))
	}

	pub fn new_arithmetic(tokens: VecDeque<Token>, vtype: VType, op: ArithmeticOperation, left: Expression, right: Expression) -> Self
	{
		Self::new(Box::new(ArithmeticExpression::new(tokens, vtype, op, left, right)))
	}

	pub fn new_comparison(tokens: VecDeque<Token>, op: ComparisonOperation, left: Expression, right: Expression) -> Self
	{
		Self::new(Box::new(ComparisonExpression::new(tokens, op, left, right)))
	}

	pub fn new_boolean(tokens: VecDeque<Token>, op: BooleanOperation, left: Expression, right: Expression) -> Self
	{
		Self::new(Box::new(BooleanExpression::new(tokens, op, left, right)))
	}

	// As function:
	pub fn as_expression<T: 'static>(&self) -> Option<&T>
	{
		self.expression.as_any().downcast_ref::<T>()
	}
}
