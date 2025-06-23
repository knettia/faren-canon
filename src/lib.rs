pub mod data;
pub mod ast;
pub mod parser;

#[cfg(test)]
mod tests
{
	use crate::ast::statement::*;

	use crate::data::vtype::*;

	use crate::parser::*;

	#[test]
	fn function_define()
	{
		let source = "
			function square(x int) int
			{
				return x * x;
			}
		";

		let (root, errors) = parse_root(source.into());

		assert_eq!(errors.len(), 0);

		let statements = root.statements;
		let func_define_statement = &statements[0];

		assert_eq!(func_define_statement.stype(), StatementType::FunctionDefine);

		let func_define_statement = func_define_statement.as_statement::<FunctionDefineStatement>().unwrap();
		let func_sign = func_define_statement.signature();

		assert_eq!(func_sign.name(), "square");
		assert_eq!(func_sign.return_type(), VType::Integer);
	}

	#[test]
	fn function_declare()
	{
		let source = "
			function square(x int) int;
		";

		let (root, errors) = parse_root(source.into());

		assert_eq!(errors.len(), 0);

		let statements = root.statements;
		let func_declare_statement = &statements[0];

		assert_eq!(func_declare_statement.stype(), StatementType::FunctionDeclare);

		let func_declare_statement = func_declare_statement.as_statement::<FunctionDeclareStatement>().unwrap();
		let func_sign = func_declare_statement.signature();

		assert_eq!(func_sign.name(), "square");
		assert_eq!(func_sign.return_type(), VType::Integer);
	}

	#[test]
	fn global_state_err()
	{
		let source = "
			let x int = 10;
		";

		let (_, errors) = parse_root(source.into());

		assert_ne!(errors.len(), 0);
	}
}
