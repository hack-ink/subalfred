// crates.io
use camino::Utf8PathBuf;

pub fn swap_file_path(path: &Utf8PathBuf) -> Option<Utf8PathBuf> {
	let file_name = path.file_name()?;

	Some(path.with_file_name(format!(".{file_name}.swp")))
}
