use super::*;
use crate::prelude::*;
use crate::log;
use crate::utils;
use crate::colours::*;

use path_macro::path;
use time::UtcDateTime;

use std::fs::File;
use std::io::{ self, BufWriter, Write };


impl Renderer<'_>
{
	/// Render `+page.ts` for supplying page metadata to SvelteKit.
	pub(super) fn render_page_ts(&self,
		wrote: &mut bool,
		dry_run: bool,
	) -> SquarkResult
	{
		let (base, folder) = utils::split_path_from(&self.dest_folder, &self.config.paths.root);

		log::info!(
			"{} to: {base}/{GREY1}{folder}{GREY}/+page.ts",
			if dry_run {"would render"} else {"rendering"},
		);
		
		let dest = path!(self.dest_folder / "+page.ts");
		
		if dest.exists() {
			self.err_exists(&dest)?;
		}

		if dry_run {
			self.write_page_ts(BufWriter::new(io::sink()))?;
		} else {
			self.write_page_ts(BufWriter::new(File::create(dest)?))?;
		}

		*wrote = true;
		Ok(())
	}

	fn write_page_ts(&self, mut f: impl Write) -> SquarkResult
	{
		let now = UtcDateTime::now();
		write!(f,
			"// rendered by Squarkdown on {YYYY} {M} {D} at {hh:02}:{mm:02}\n\n",
			YYYY = now.year(), M = now.month(), D = now.day(),
			hh = now.hour(), mm = now.minute(),
		)?;

		let p = self.page;
		let s = self.config.out.shorter_fields;

		f.write_all(b"import type { PageData } from \"stranger-quarkdown\";\n\n")?;
		
		if !p.other.is_empty() {
			writeln!(f, "type OtherFields =")?;
			for key in p.other.keys() {
				writeln!(f, "\t| \"{key}\"")?;
			}
			writeln!(f, ";\n")?;
		}

		write!(f, "export function load(): PageData<\"{}\">", if s { "short" } else { "long" })?;

		if !p.other.is_empty() {
			write!(f, " & Record<OtherFields, string[]>")?;
		}

		f.write_all(b" {\n")?;
		f.write_all(b"\treturn {\n")?;

		writeln!(f, "\t\tshard: {:?},", &p.shard)?;

		let path = utils::display_rel(&p.filepath, &self.config.paths.root);
		writeln!(f, "\t\t{}: {:?},", p.path(s), path)?;

		let dest = utils::display_rel(&p.destination, &self.config.out.folder);
		writeln!(f, "\t\t{}: {:?},", p.dest(s), dest)?;
		
		writeln!(f, "\t\t{}: {:?},", p.flags(s), p.flags)?;

		if !p.title.is_empty()       { writeln!(f, "\t\t{}: {:?},", p.title(s), p.title )?; }
		if !p.description.is_empty() { writeln!(f, "\t\t{}: {:?},", p.desc(s),  p.description )?; }
		if !p.heading.is_empty()     { writeln!(f, "\t\t{}: {:?},", p.head(s),  p.heading )?; }
		if !p.caption.is_empty()     { writeln!(f, "\t\t{}: {:?},", p.capt(s),  p.caption )?; }

		writeln!(f, "\t\t{}: {:?},", p.tags(s), p.tags)?;

		if let Some(v) = p.release_date {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				p.date(s),
				v.year(), v.month() as u8 - 1, v.day()
			)?;
		}
		if !p.release_date_raw.is_empty() {
			writeln!(f, "\t\t{}: {:?},", p.date_raw(s), p.release_date_raw)?;
		}
		if let Some(v) = p.last_update {
			writeln!(f,
				"\t\t{}: new Date({}, {}, {}),",
				p.update(s),
				v.year(), v.month() as u8 - 1, v.day()
			)?;
		}
		if !p.last_update_raw.is_empty() {
			writeln!(f, "\t\t{}: {:?},", p.update_raw(s), p.last_update_raw)?;
		}

		for (key, val) in &p.other {
			writeln!(f, "\t\t\"{key}\": {val:?},")?;
		}

		f.write_all(b"\t};\n")?;
		f.write_all(b"}\n")?;

		Ok(())
	}
}
