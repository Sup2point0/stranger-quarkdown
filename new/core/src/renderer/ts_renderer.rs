use super::*;
use crate::prelude::*;
use crate::log;
use crate::utils;
use crate::colours::*;

use path_macro::path;

use std::fs::File;
use std::io::{ BufWriter, Write };


impl Renderer<'_>
{
	/// Render `+page.ts` for supplying page data to SvelteKit.
	pub(super) fn render_page_ts(&self) -> SquarkResult
	{
		log::info!(
			"rendering to: {GREY1}{}{GREY}/+page.ts",
			utils::display_rel(&self.dest_folder, &self.config.paths.root),
		);
		
		let dest = path!(self.dest_folder / "+page.ts");
		
		if dest.exists() {
			self.err_exists(&dest)?;
		}

		let mut f = BufWriter::new(File::create(dest)?);

		f.write_all(b"import type { PageData } from \"squarkdown\";\n\n")?;
		f.write_all(b"export function load(): PageData {\n")?;
		f.write_all(b"\treturn {\n")?;

		let d = self.page.serialised_long(self.config);
		let s = self.config.out.shorter_fields;

		writeln!(f, "\t\t{}: {:?},", d.filepath(s), d.filepath)?;
		writeln!(f, "\t\t{}: {:?},", d.dest(s),     d.destination)?;
		writeln!(f, "\t\t{}: {},",   d.flags(s),    d.flags)?;

		if let Some(v) = &d.title {
			writeln!(f, "\t\t{}: {v:?},", d.title(s))?;
		}
		if let Some(v) = &d.description {
			writeln!(f, "\t\t{}: {v:?},", d.desc(s))?;
		}
		if let Some(v) = &d.heading {
			writeln!(f, "\t\t{}: {v:?},", d.head(s))?;
		}
		if let Some(v) = &d.caption {
			writeln!(f, "\t\t{}: {v:?},", d.capt(s))?;
		}

		writeln!(f, "\t\ttags: {:?},", d.tags)?;

		if let Some(v) = d.release_date {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				d.date(s),
				v.year(), v.month() as u8, v.day()
			)?;
		}
		if let Some(v) = d.release_date_raw {
			writeln!(f, "\t\t{}: {v:?},", d.date_raw(s))?;
		}
		if let Some(v) = d.last_update {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				d.update(s),
				v.year(), v.month() as u8, v.day()
			)?;
		}
		if let Some(v) = d.last_update_raw {
			writeln!(f, "\t\t{}: {v:?},", d.update_raw(s))?;
		}

		writeln!(f, "\t\t{}: {:?},", d.other(s), d.other)?;

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
