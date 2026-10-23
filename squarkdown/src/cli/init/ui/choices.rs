macro_rules! choices
{
	(
		$variant:ident $(= $display:literal)?
		$(
			, $rest:ident $(= $displays:literal)?
		)*
		$(,)?
	) =>
	{
		#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_display::EnumDisplay)]
		#[enum_display(case = "Title")]
		enum _Choice
		{
			$(#[display($display)])?
			$variant,
			$(
				$(#[display($displays)])?
				$rest,
			)*
		}

		use _Choice::$variant;
		$(use _Choice::$rest;)*
	}
}
pub(crate) use choices;
