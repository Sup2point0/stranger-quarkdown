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
		let p = self.page;
		let s = self.config.out.shorter_fields;

		f.write_all(b"import type { PageData } from \"stranger-quarkdown\";\n\n")?;
		f.write_all(b"export function load(): PageData<")?;
		write!(f, "{}", if s { "\"short\"" } else { "\"long\"" })?;
		f.write_all(b"> {\n")?;
		f.write_all(b"\treturn {\n")?;

		let path = utils::display_rel(&p.filepath, &self.config.paths.root);
		writeln!(f, "\t\t{}: {:?},", p.path(s), path)?;

		let dest = utils::display_rel(&p.destination, &self.config.out.folder);
		writeln!(f, "\t\t{}: {:?},", p.dest(s), dest)?;
		
		writeln!(f, "\t\t{}: {:?},", p.flags(s), p.flags)?;

		if let Some(v) = &p.title       { writeln!(f, "\t\t{}: {v:?},", p.title(s))?; }
		if let Some(v) = &p.description { writeln!(f, "\t\t{}: {v:?},", p.desc(s))?; }
		if let Some(v) = &p.heading     { writeln!(f, "\t\t{}: {v:?},", p.head(s))?; }
		if let Some(v) = &p.caption     { writeln!(f, "\t\t{}: {v:?},", p.capt(s))?; }

		writeln!(f, "\t\ttags: {:?},", p.tags)?;

		if let Some(v) = p.release_date {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				p.date(s),
				v.year(), v.month() as u8 - 1, v.day()
			)?;
		}
		if let Some(v) = &p.release_date_raw {
			writeln!(f, "\t\t{}: {v:?},", p.date_raw(s))?;
		}
		if let Some(v) = p.last_update {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				p.update(s),
				v.year(), v.month() as u8 - 1, v.day()
			)?;
		}
		if let Some(v) = &p.last_update_raw {
			writeln!(f, "\t\t{}: {v:?},", p.update_raw(s))?;
		}

		writeln!(f, "\t\t{}: {:?},", p.other(s), p.other)?;

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
