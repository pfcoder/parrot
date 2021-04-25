use crate::form_modal::Forms;
use chrono::{DateTime, Utc};
use csv;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

pub fn form_store(data: &Forms) -> Result<(), Box<dyn Error>> {
	// get current date
	let now: DateTime<Utc> = Utc::now();
	let path = format!("./data/{}/{}.csv", data.get_type(), now.format("%Y-%m-%d"));
	let is_new = !Path::new(&path).exists();
	println!("csv-name:{} is_new:{}", path, is_new);
	let file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(&path)?;

	let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

	// get header from model
	if is_new {
		writer.write_record(&data.get_header())?;
	}

	writer.serialize(data)?;

	writer.flush()?;

	Ok(())
}
