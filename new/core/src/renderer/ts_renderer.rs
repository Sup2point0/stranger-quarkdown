use super::*;
use crate::core::*;
use crate::log;
use crate::utils;
use crate::colours::*;

use std::fs::File;
use std::io::{ BufWriter, Write };


impl Renderer<'_>
{
	/// Render `+page.ts` for supplying page data to SvelteKit.
	pub(super) fn render_page_ts(&mut self) -> SquarkResult
	{
		log::info!(
			"rendering to: {GREY1}{}{GREY}/+page.ts",
			utils::display_rel(&self.dest_folder, &self.config.paths.root),
		);

		let file = File::create(self.dest_folder.join("+page.ts"))?;
		let mut f = BufWriter::new(file);

		f.write_all(b"import type { PageData } from 'squarkdown';\n\n")?;
		f.write_all(b"export function load(): PageData {\n")?;
		f.write_all(b"\treturn {\n")?;

		let d = self.page.clone().serialise(self.config);
		let s = self.config.out.shorter_fields;

		writeln!(f, "\t\t{}: {:?},", d.filepath(s),    d.filepath)?;
		writeln!(f, "\t\t{}: {:?},", d.destination(s), d.destination)?;
		writeln!(f, "\t\t{}: {},",   d.flags(s),       d.flags)?;

		if let Some(title) = &d.title {
			writeln!(f, "\t\t{}: {title:?},", d.title(s))?;
		}
		if let Some(description) = &d.description {
			writeln!(f, "\t\t{}: {description:?},", d.description(s))?;
		}
		if let Some(heading) = &d.heading {
			writeln!(f, "\t\t{}: {heading:?},", d.heading(s))?;
		}
		if let Some(caption) = &d.caption {
			writeln!(f, "\t\t{}: {caption:?},", d.caption(s))?;
		}

		writeln!(f, "\t\ttags: {:?},", d.tags)?;

		if let Some(release_date) = d.release_date {
			writeln!(f, "\t\t{}: {release_date:?},", d.release_date(s))?;
		}
		if let Some(last_updated) = d.last_updated {
			writeln!(f, "\t\t{}: {last_updated:?},", d.last_updated(s))?;
		}

		writeln!(f, "\t\t{}: {:?},", d.other(s), d.other)?;

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
