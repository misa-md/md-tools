/**
 * this file is copied from xyzio package,
 * but add `rayon` parallel support for reading file
 */

use std::io;
use std::io::prelude::BufRead;
use std::iter::Iterator;

use xyzio::{Snapshot, Atom};
use rayon::prelude::*;

pub struct Reader<R> {
    reader: io::BufReader<R>,
}

macro_rules! parse_line {
    ($reader:ident) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer
    }};
    ($reader:ident, $t:ty) => {{
        let mut buffer = String::new();
        $reader.read_line(&mut buffer)?;
        buffer.trim().parse::<$t>()?
    }}
}

impl<R: io::Read> Reader<R> {
    /** we suggest use [`std::fs::File`] as [`inner`] R.
     * Because all data will be read into memory from inner first and then perform parsing.
     * For example, if we use memory bytes array as `inner`,
     * then there are two copies in memory (one is in inner and another one is saved before parsing),
     * which will cause memory wasting.
     */
    pub fn new(inner: R) -> Self {
        Reader {
            reader: io::BufReader::new(inner)
        }
    }

    // read and parse data in xyz file in parallel.
    pub fn read_snapshot(&mut self) -> Result<Snapshot, xyzio::Error> {
        let reader = &mut self.reader;

        let num_atoms = parse_line!(reader, usize);
        let comment = parse_line!(reader);

        // read all lines into memory first.
        // all lines in xyz file (except for file header).
        let mut atoms_lines: Vec<String> = Vec::new();
        for _ in 0..num_atoms {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            atoms_lines.push(buffer);
        }

        // parsing data in parallel.
        // let mut atoms: Vec<Atom> = Vec::new();
        let atoms: Vec<_> = (0..num_atoms).into_par_iter().map(|i| {
            atoms_lines[i].trim().parse::<Atom>().unwrap()
        }).collect();

        Ok(Snapshot {
            comment: comment,
            atoms: atoms,
        })
    }
}

impl<R: io::Read> Iterator for Reader<R> {
    type Item = Snapshot;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_snapshot().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let data: &[u8] = b"\
            3
            comment
            C 1.0 2.0 3.0
            O 4.0 3.0 6.0
            H 5.0 1.5 4.0";
        let mut reader = Reader::new(data);
        let success = reader.read_snapshot();
        assert!(success.is_ok());

        let snapshot = success.unwrap();
        assert_eq!(3, snapshot.size());
    }

    #[test]
    fn test_iterator() {
        let data: &[u8] = b"\
            3
            1st snapshot
            C 1.0 2.0 3.0
            O 4.0 3.0 6.0
            H 5.0 1.5 4.0
            3
            2nd snapshot
            C 1.1 1.9 2.8
            O 4.2 3.0 5.9
            H 5.0 1.6 4.0";
        let mut reader = Reader::new(data);
        assert!(reader.next().is_some());
        assert!(reader.next().is_some());
        assert!(reader.next().is_none());
    }
}
