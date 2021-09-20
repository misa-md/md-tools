use std::fs::File;
use std::io::{Read, Seek};

use byte_struct::ByteStruct;

use crate::conv::binary_types;
use crate::conv::binary_parser::{ParseError};
use crate::conv::v2_atom_types;

pub struct BinaryParserV2 {
    next_frame: u32,
    // current processing rank
    cur_rank: u32,
    // current index in block
    cur_index_in_block: u32,
    // rank data offset, and block index(how many atoms walked in this block)
    rank_start_offset: Vec<(u64, u32)>,
    // temp data of atom
    atom: binary_types::TypeAtom,
    pub(crate) global_header: v2_atom_types::GlobalMetaData,
    // current frame in processing
    file: File,
}

// initialize parser
pub fn make_parser(filename: &str)
                   -> std::result::Result<BinaryParserV2, ParseError> {
    let mut input_file = File::open(filename).unwrap();

    let global_header_size = std::mem::size_of::<v2_atom_types::GlobalMetaData>();
    let mut buffer = [0; std::mem::size_of::<v2_atom_types::GlobalMetaData>()];

    // read global header
    let n = match input_file.read(&mut buffer[..]) {
        Ok(p) => p,
        Err(_e) => return Err(ParseError),
    };

    // parse global header
    if n != global_header_size {
        println!("bad global header size. file format version may be not incompatible");
        return Err(ParseError);
    } else {
        let global_header: v2_atom_types::GlobalMetaData = v2_atom_types::GlobalMetaData::read_bytes(&buffer[..]);

        // create and initialize block offset vector
        let mut rank_offset_vec = Vec::with_capacity(global_header.mpi_ranks as usize);
        rank_offset_vec.resize(global_header.mpi_ranks as usize, (0, 0_u32));
        for i in 0..rank_offset_vec.len() {
            let base_cursor: u64 = global_header.self_size + (global_header.frames as u64) * global_header.frame_meta_size
                + global_header.mpi_ranks * global_header.local_size;
            rank_offset_vec[i] = (base_cursor + (i as u64) * global_header.block_atoms * global_header.atom_item_bytes, 0_u32);
        };

        let binary_parser = BinaryParserV2 {
            next_frame: 0,
            cur_rank: 0,
            cur_index_in_block: 0,
            rank_start_offset: rank_offset_vec,
            file: input_file,
            global_header,
            atom: binary_types::TypeAtom {
                id: 0,
                tp: 0,
                inter_type: 0,
                atom_location: [0.0, 0.0, 0.0],
                atom_velocity: [0.0, 0.0, 0.0],
                atom_force: [0.0, 0.0, 0.0],
            },
        };
        return Ok(binary_parser);
    };
}

impl BinaryParserV2 {
    fn parse_from_binary(&mut self, buffer: &mut [u8]) {
        let atom_data = v2_atom_types::AtomInfoDump::read_bytes(&buffer[..]);
        self.atom.id = atom_data.id;
        self.atom.tp = atom_data.type_;

        let mut cursor = v2_atom_types::AtomInfoDump::size_in_file();

        if self.global_header.mask & (1 << 0) != 0 {
            let left: &[u8] = &buffer[cursor..];
            let atom_pos = v2_atom_types::AtomDumpData3D::read_bytes(&left[..]);
            self.atom.atom_location = atom_pos.atom_props;
            cursor += std::mem::size_of::<v2_atom_types::AtomDumpData3D>();
        }
        if self.global_header.mask & (1 << 1) != 0 {
            let left: &[u8] = &buffer[cursor..];
            let atom_v = v2_atom_types::AtomDumpData3D::read_bytes(&left[..]);
            self.atom.atom_velocity = atom_v.atom_props;
            cursor += std::mem::size_of::<v2_atom_types::AtomDumpData3D>();
        }
        if self.global_header.mask & (1 << 2) != 0 {
            let left: &[u8] = &buffer[cursor..];
            let atom_f = v2_atom_types::AtomDumpData3D::read_bytes(&left[..]);
            self.atom.atom_force = atom_f.atom_props;
            std::mem::size_of::<v2_atom_types::AtomDumpData3D>();
        }
    }
    // try to checkout into next block
    fn try_switch_to_next_block(&mut self) {
        if (self.cur_index_in_block as u64) >= self.global_header.block_atoms {
            let seek_size = self.global_header.block_atoms * (self.global_header.mpi_ranks - 1) * self.global_header.atom_item_bytes;
            self.file.seek(std::io::SeekFrom::Current(seek_size as i64));
            self.cur_index_in_block = 0;
        }
        self.cur_index_in_block += 1;
    }

    fn switch_to_next_rank(&mut self) {
        // record start offset in file for next frame
        self.rank_start_offset[self.cur_rank as usize] = (self.file_tell(), self.cur_index_in_block);
        self.cur_rank = (self.cur_rank + 1) % (self.global_header.mpi_ranks as u32); // just like a ring: [0, mpi_ranks-1]

        // recover offset and index in block in the new rank (useful if there are multiple frames in file).
        let offset = self.rank_start_offset[self.cur_rank as usize];
        self.file.seek(std::io::SeekFrom::Start(offset.0));
        self.cur_index_in_block = offset.1;
    }

    fn file_tell(&mut self) -> u64 {
        return self.file.seek(std::io::SeekFrom::Current(0)).unwrap();
    }
}

impl binary_types::BinaryParser for BinaryParserV2 {
    fn global_header(&self) -> u32 {
        println!("file metadata:");
        println!("{:#?}", self.global_header);
        return self.global_header.frames;
    }

    // read next atom in one frame
    fn next(&mut self) -> bool {
        self.try_switch_to_next_block();

        // prepare for reading one atom
        let atom_data_size = self.global_header.atom_item_bytes as usize;
        let mut buffer = vec![0; atom_data_size];

        // read one atom
        let _n = match self.file.read(&mut buffer[..]) {
            Ok(p) => p,
            Err(e) => panic!("{:?}", e),
        };

        // parse it, the result is in self.atom.
        self.parse_from_binary(&mut buffer[0..atom_data_size]);

        if self.atom.tp == -1 {
            let rank_before_switch = self.cur_rank;
            // switch to next rank. change "cur_rank" and "index cursor in block" for next rank.
            self.switch_to_next_rank();
            if ((rank_before_switch + 1) as u64) >= self.global_header.mpi_ranks {
                // current frame end
                println!("current frame end");
                return false;
            }
        }
        return true;
    }

    fn decode(&mut self) -> binary_types::TypeAtom {
        return self.atom;
    }

    fn move_to_next_frame(&mut self) -> bool {
        if self.next_frame >= self.global_header.frames {
            return false;
        }
        // move file cursor to 1st rank.
        self.cur_rank = 0;
        let cursor: u64 = self.rank_start_offset[0].0; // move to first rank and first block
        self.cur_index_in_block = self.rank_start_offset[0].1;
        self.file.seek(std::io::SeekFrom::Start(cursor));
        self.next_frame += 1;
        return true;
    }

    fn frame_header(&self) {}

    fn close(&self) {}
}
