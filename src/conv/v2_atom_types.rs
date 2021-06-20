use byte_struct::*;

// c side type: ::std::os::raw::c_ulong;
pub type SizeT = u64;

// C side type: ::std::os::raw::c_ulong;
pub type TypeAtomId = u64;
// C side type: ::std::os::raw::c_int;
pub type TypeAtomType = i32;
// C side type: ::std::os::raw::c_short;
// pub type TypeInterType = i16;
// pub type TypeAtomLocation = f64;
// pub type TypeAtomVelocity = f64;
// pub type TypeAtomForce = f64;
// C side type: ::std::os::raw::c_uint;
pub type TypeDumpMask = u32;
// c side type: ::std::os::raw::c_uint
pub type TypeFormatVersion = u32;
// C side type: ::std::os::raw::c_uint
pub type TypeFrames = u32;

#[repr(C)]
#[derive(ByteStruct, PartialEq)]
#[derive(Debug, Copy, Clone)]
#[byte_struct_le]
pub struct GlobalMetaData {
    pub self_size: SizeT,
    pub frame_meta_size: SizeT,
    pub block_atoms: SizeT,
    pub atoms_num: SizeT,
    pub atom_item_bytes: SizeT,
    pub mpi_ranks: SizeT,
    pub mask: TypeDumpMask,
    pub format_version: TypeFormatVersion,
    pub global_header_size: SizeT,
    pub local_size: SizeT,
    pub frames: TypeFrames,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrameMetaData {
    pub atoms_num: SizeT,
    pub atoms_num_hash_collision: SizeT,
    pub step: SizeT,
    pub time: f64,
}

#[repr(C)]
#[derive(ByteStruct, PartialEq)]
#[derive(Debug, Copy, Clone)]
#[byte_struct_le]
pub struct AtomInfoDump {
    pub id: TypeAtomId,
    pub type_: TypeAtomType,
}

impl AtomInfoDump {
    // the data in file is saved in compact mode
    pub(crate) fn size_in_file() -> usize {
        // it is 8 + 4 = 12, not 16 (sizeof<AtomInfoDump>() is 16).
        return std::mem::size_of::<TypeAtomId>() + std::mem::size_of::<TypeAtomType>();
    }
}

#[repr(C)]
#[derive(ByteStruct, PartialEq)]
#[derive(Debug, Copy, Clone)]
#[byte_struct_le]
pub struct AtomDumpData3D {
    pub atom_props: [f64; 3usize],
}
