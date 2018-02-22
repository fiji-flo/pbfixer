use std::ffi::OsStr;
use std::fs::File;
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use protobuf;
use protobuf::core::MessageStatic;

pub fn read_pb_file<T: MessageStatic>(file: PathBuf) -> Result<Vec<T>, String> {
    let f = File::open(file).map_err(|e| format!("unable to open file: {}", e))?;
    let mut buf_reader = BufReader::new(f);
    let mut in_stream = protobuf::CodedInputStream::from_buffered_reader(&mut buf_reader);
    let mut msgs = Vec::new();
    while let Ok(msg) = protobuf::core::parse_length_delimited_from::<T>(&mut in_stream) {
        msgs.push(msg);
    }
    Ok(msgs)
}

pub fn write_pb_file<T: MessageStatic>(file: PathBuf, msgs: Vec<T>) -> Result<usize, String> {
    fs::create_dir_all(file.parent().unwrap())
        .map_err(|_| format!("unable to create {:?}", file.parent()))?;
    let f = File::create(&file).map_err(|e| format!("unable to open file: {} ({:?})", e, file))?;
    let mut count = 0;
    let mut w = BufWriter::new(f);
    let mut os = protobuf::CodedOutputStream::new(w.get_mut());
    for msg in msgs {
        msg.write_length_delimited_to(&mut os)
            .map_err(|e| format!("{}", e))?;
        count += 1;
    }
    os.flush()
        .map_err(|e| format!("flush it real bad 1: {}", e))?;
    Ok(count)
}

pub fn run<T: MessageStatic>(
    in_dir: &str,
    out_dir: &str,
    update: &Fn(Vec<T>) -> Result<(Vec<T>, usize), String>,
) -> Result<(), String> {
    let mut total = 0;
    let mut total_msgs = 0;
    for date in fs::read_dir(in_dir).map_err(|e| format!("cannot read dir: {} ({})", in_dir, e))? {
        let date = date.map_err(|e| format!("{}", e))?;
        println!("processing {:?}", date);
        for file in
            fs::read_dir(date.path()).map_err(|e| format!("cannot read dir: {} ({})", in_dir, e))?
        {
            let file = file.map_err(|e| format!("{}", e))?;
            let (msgs, count) = update(read_pb_file(file.path())?)?;
            total += count;
            let out: PathBuf = [
                OsStr::new(out_dir),
                date.path()
                    .file_name()
                    .ok_or_else(|| format!("unable to get filename for {:?}", date))?,
                file.path()
                    .file_name()
                    .ok_or_else(|| format!("unable to get filename for {:?}", date))?,
            ].iter()
                .collect();
            let written = write_pb_file(out, msgs)?;
            total_msgs += written;
        }
        println!("… {}/{}", total, total_msgs);
    }
    println!("TOTAL {}/{}", total, total_msgs);
    Ok(())
}
