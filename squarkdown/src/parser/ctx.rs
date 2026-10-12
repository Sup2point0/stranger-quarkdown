use crate::types::ContextStack;

use enum_display::EnumDisplay;


#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, EnumDisplay)]
pub enum ParseCtx
{
	#[default]
	#[display("parsing charm squark")]
	CHARM_SQUARK,

	#[display("extracting <h1> heading")]
	HEADING,

	#[display("parsing flags")]
	FLAGS,

	#[display("parsing fields")]
	FIELDS,

	#[display("parsing a field")]
	FIELD,

	#[display("parsing identifier")]
	IDENT,

	#[display("parsing values")]
	VALUES,
}


impl ContextStack<ParseCtx>
{
	pub fn printed(&self) -> Vec<String> {
		self.printed_with_prefix("while")
	}
}


macro_rules! ctx
{
	($self:ident, $ctx:expr => $body:block) => {
		{
			$self.ctx.push($ctx);
			let r = (|| { $body })();
			$self.ctx.try_pop(&$ctx)?;
			r
		}
	};
}
pub(crate) use ctx;
