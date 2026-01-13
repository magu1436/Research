use std::fs::{File, create_dir_all};
use std::path::Path;
use std::io::{BufWriter, Write};
use std::vec;
use arrayfire::Array;

pub fn write_into_csv(val: &Array<f64>) -> std::io::Result<()> {

    const FILE_PATH: &str = "result/result.csv";
    let path = Path::new(FILE_PATH);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let n = val.elements() as usize;
    let mut v = vec![0.0f64; n];
    val.host(&mut v);

    let line = v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    writeln!(writer, "{line}")?;

    Ok(())
}