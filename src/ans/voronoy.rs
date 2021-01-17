use std::{f32};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::collections::BTreeMap;
use std::error::Error;

pub type Float = f32;
type Inx = i32;

pub const LATTICE_CONST: Float = 2.85532;

/**
 * The coordinate offset of 8 minor lattices surrounding a major lattice.
 * If the major lattice is determined, we can get the lattice coordinate of a minor lattice
 * by this offset.
 */
const OFFSET: [(Inx, Inx, Inx); 8] = [ //offset index: 0-> x, 1 -> y, 2 -> z
    (-1, -1, -1), // x low,  y low,  z low, (b000)
    (1, -1, -1), // x high, y low,  z low, (b001)
    (-1, 0, -1), // x low,  y high, z low, (b010)
    (1, 0, -1), // x high, y high, z low, (b011)
    (-1, -1, 0),  // x low,  y low,  z high,(b100)
    (1, -1, 0),  // x high, y low,  z high,(b101)
    (-1, 0, 0),  // x low,  y high, z high,(b110)
    (1, 0, 0),  // x high, y high, z high,(b111)
];

/**
 * Assuming plane: Ax+By+Cz+d = 0.
 * For major lattice with coordinate: (0,0,0), it has 8 1nn lattices.
 * Then, we can compute the 8 vertical bisector planes between major lattice and each 1nn lattices:
 * 0: -x-y-z-3/4 = 0;
 * 1: x-y-z-3/4 = 0;
 * 2: -x+y-z-3/4 = 0;
 * 3: x+y-z-3/4 = 0;
 * 4: -x-y+z-3/4 = 0;
 * 5: x-y+z-3/4 = 0;
 * 6: -x+y+z-3/4 = 0;
 * 7: x+y+z-3/4 = 0;
 *
 * This array saves the normal vectors of 8 vertical bisector planes.
 *
 * Noticing that, for major lattice position:(0, 0, 0), we always have A*0+B*0+C*0+d < 0.
 */
const NORMAL_VECTOR: [(Float, Float, Float); 8] = [
    (-1.0, -1.0, -1.0), // normal vector of vertical bisector plane of (-0.5, -0.5, -0.5) and (0,0,0)
    (1.0, -1.0, -1.0), // (0.5,  -0.5, -0.5) and (0,0,0)
    (-1.0, 1.0, -1.0), // (-0.5, 0.5, -0.5)  and (0,0,0)
    (1.0, 1.0, -1.0), // (0.5, 0.5, -0.5)  and (0,0,0)
    (-1.0, -1.0, 1.0),  // (-0.5, -0.5, 0.5)  and (0,0,0)
    (1.0, -1.0, 1.0),  // (0.5,  -0.5, 0.5)  and (0,0,0)
    (-1.0, 1.0, 1.0),  // (-0.5, 0.5,  0.5)  and (0,0,0)
    (1.0, 1.0, 1.0),  // (0.5,  0.5, 0.5)  and (0,0,0)
];
/**
  * d = -(A*x_m + B*y_m + C*z_m), in which (A,B,C) is normal vector (such as (-1, -1, -1) ),
  * (x_m, y_m, z_m) is middle pointer (such as (-1/4, -1/4, -1/4)).
  */
const D: Float = -3.0 / 4.0;

const ANALYSIS_OUT_FILE_HEADER: &str = "type, lattice:x, lattice;y, lattice:z,\
 position:x, position:y, position:z\
";

pub fn do_analysis_wrapper(output: &str, box_size: (usize, usize, usize), box_start: (Float, Float, Float), mut snapshot: &xyzio::Snapshot) {
    // prepare file
    let path = Path::new(output);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(file);
    writer.write(ANALYSIS_OUT_FILE_HEADER.as_bytes()).unwrap();
    do_analysis(&mut writer, box_size, box_start, &snapshot.atoms);
    writer.flush().unwrap();
}

// in do_analysis, calculate atom's occupation by box size and its lattice index,
// then atoms with >= 2 occupation will be logged.
fn do_analysis(writer: &mut BufWriter<File>, (box_x_size, box_y_size, box_z_size): (usize, usize, usize), (box_x_start, box_y_start, box_z_start): (Float, Float, Float), atoms: &Vec<xyzio::Atom>) {
    // scale to lattice const unit.
    let mut atoms_lat_map = BTreeMap::new();

    let atoms_size = atoms.len();
    for i in 0..atoms_size {
        // calculate lattice index of each atom
        // note: x is doubled.
        let (x, y, z) = voronoy(atoms[i].x - box_x_start, atoms[i].y - box_y_start, atoms[i].z - box_z_start);
        // todo mode box_x/y/z_size if index is large then box size (todo for user specified box size, not auto size).
        // todo or mode box_x/y/z_size if real box coord not starting from 0.
        // which also means: make x,y,z belongs [0, box_x/y/z_size).
        // z * 2 * x_size * y_size  + y * 2 * x_size  + x;
        let global_index = 2 * (box_x_size as Inx) * (z * box_y_size as Inx + y) + x;

        match atoms_lat_map.get(&global_index) {
            Some(&atom_index) => {
                if atom_index == -1 {
                    // output itself only (not write first item)
                    write_line(writer, global_index, &atoms[i], (box_x_size, box_y_size));
                } else {
                    // output i self and first item at the same index in map
                    write_line(writer, global_index, &atoms[atom_index as usize], (box_x_size, box_y_size));
                    write_line(writer, global_index, &atoms[i], (box_x_size, box_y_size));
                    atoms_lat_map.insert(global_index, -1); // tag it as "already written"
                }
            }
            None => {
                atoms_lat_map.entry(global_index).or_insert(i as Inx);
            }
        }
    }

    // log vacancy
    for i in 0..atoms_size {
        let global_index = i as Inx;
        match atoms_lat_map.get(&global_index) {
            Some(&atom_index) => {
                // skip
            }
            None => {
                let atom = xyzio::Atom {
                    element: "V".to_string(),
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                write_line(writer, global_index, &atom, (box_x_size, box_y_size))
            }
        }
    }
}

// write a line to file by passing global lattice index and atom information
fn write_line(writer: &mut BufWriter<File>, global_index: Inx, atom: &xyzio::Atom, (box_x_size, box_y_size): (usize, usize)) {
    let (_box_size_x, _box_size_y) = (box_x_size as Inx, box_y_size as Inx);
    let lat_z = global_index / (2 * _box_size_x * _box_size_y);
    let lat_left = global_index % (2 * _box_size_x * _box_size_y);
    let lat_y = lat_left / (2 * _box_size_x);
    let lat_x = lat_left % (2 * _box_size_x);
    writer.write(format!("{}, {}, {}, {}, {}, {}, {}\n",
                         atom.element,
                         lat_x, lat_y, lat_z,
                         atom.x, atom.y, atom.z).as_bytes()).unwrap();
}

/**
 * calculate coordinate of nearest lattice for a atom:
 * X,Y,Z is the position of atom.
 */
pub fn voronoy(x: Float, y: Float, z: Float) -> (Inx, Inx, Inx) {
    let lat_coord_x = (x / LATTICE_CONST).round() as Inx;
    let mut lat_coord_y = (y / LATTICE_CONST).round() as Inx;
    let mut lat_coord_z = (z / LATTICE_CONST).round() as Inx;

    let delta_x = x / LATTICE_CONST - (lat_coord_x as Float);
    let delta_y = y / LATTICE_CONST - (lat_coord_y as Float);
    let delta_z = z / LATTICE_CONST - (lat_coord_z as Float);

    let flag_index = (if delta_z > 0.0 { 4 } else { 0 })
        | (if delta_y > 0.0 { 2 } else { 0 })
        | (if delta_x > 0.0 { 1 } else { 0 });

    let mut lat_coord_x = 2 * lat_coord_x; // x is doubled.

    if NORMAL_VECTOR[flag_index].0 * delta_x +
        NORMAL_VECTOR[flag_index].1 * delta_y +
        NORMAL_VECTOR[flag_index].2 * delta_z + D >= 0.0 {
        /* belongs to major lattice. */
        lat_coord_x += OFFSET[flag_index].0;
        lat_coord_y += OFFSET[flag_index].1;
        lat_coord_z += OFFSET[flag_index].2;
    }
    return (lat_coord_x, lat_coord_y, lat_coord_z);
}


#[cfg(test)]
mod voronoy_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_voronoy() {
        assert_eq!(voronoy(1.377608, 1.501391, 1.471441), (1, 0, 0));
        assert_eq!(voronoy(2.772588, 0.056315, -0.044443), (2, 0, 0));
    }
}
