#[macro_use]
extern crate lazy_static;

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::{exit, Command},
};

mod license;
mod props;

use self::{
    license::License,
    props::BoolProperties,
};

const DETAILS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/licenses/json/details");
const LICENSES_OUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/spdx/gen.rs");

const KILOBYTE: usize = 1024;
const MEGABYTE: usize = KILOBYTE * KILOBYTE;

const OUT_WRITER_CAPACITY: usize = 4 * MEGABYTE;
const LICENSE_READER_CAPACITY: usize = 131_072;

fn read_licenses(dir: &Path) -> io::Result<Vec<License>> {
    let mut all = Vec::<License>::new();
    for entry in fs::read_dir(dir)? {
        let reader = io::BufReader::with_capacity(
            LICENSE_READER_CAPACITY,
            File::open(entry?.path())?,
        );
        all.push(serde_json::from_reader::<_, License>(reader)?);
    }
    all.sort_unstable_by(|a, b| a.ident.cmp(&b.ident));
    Ok(all)
}

fn read_write_data() -> io::Result<()> {
    let all = read_licenses(DETAILS_DIR.as_ref())?;

    let licenses_out = Path::new(LICENSES_OUT).canonicalize()?;
    let mut out = io::BufWriter::with_capacity(
        OUT_WRITER_CAPACITY,
        File::create(&licenses_out)?,
    );
    write!(
        out,
        include_str!("../TEMPLATE.rs"),
        license_variants = license::render::Variants(&all),
        count = all.len(),
        info = license::render::Info(&all),
        id_to_license = license::render::IdMapping(&all),
    )?;
    out.flush()?;

    // Format the output nicely
    Command::new("rustfmt")
        .arg(licenses_out.to_string_lossy().as_ref())
        .status()?;

    Ok(())
}

fn main() {
    if let Err(error) = read_write_data() {
        eprintln!("{}", error);
        exit(1);
    }
}
