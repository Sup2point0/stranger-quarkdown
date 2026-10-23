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


/// Turn one line of the `choices:` parameter of [`pick_one`] or [`pick_many`] into its internal `Pick___Choice` struct.
macro_rules! _define_choice
{
	(  @go [$($acc:expr ,)+]  ) => {
		vec![$($acc ,)+]
	};

	(
		@go [$($acc:tt)*] $value:expr => $label:expr => $hint:expr ; $($rest:tt)*
	) => {
		_define_choice!(@go
			[$($acc)*
				PickOneChoice { value: $value, label: $label, hint: $hint },
			]
			$($rest)*
		)
	};

	(
		@go [$($acc:tt)*] $value:expr => $hint:literal $(, $($args:tt)*)+ ; $($rest:tt)*
	) => {
		_define_choice!(@go
			[$($acc)*
				PickOneChoice {
					value: $value,
					label: $value,
					hint: format!($hint $(, $($args)*)+),
				},
			]
			$($rest)*
		)
	};

	(
		@go [$($acc:tt)*] $value:expr => $hint:expr ; $($rest:tt)*
	) => {
		_define_choice!(@go
			[$($acc)*
				PickOneChoice { value: $value, label: $value, hint: $hint },
			]
			$($rest)*
		)
	};

	(
		@go [$($acc:tt)*] $value:expr ; $($rest:tt)*
	) => {
		_define_choice!(@go
			[$($acc)*
				PickOneChoice {
					value: $value,
					label: $value,
					hint: String::new(),
				},
			]
			$($rest)*
		)
	};

	(  $($choices:tt)+  ) => {
		_define_choice!(@go [] $($choices)+)
	};
}
pub(crate) use _define_choice;
