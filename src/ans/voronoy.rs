use std::f32;
use std::fs::File;
use xyzio::Reader;
use std::collections::BTreeMap;

const LATTICE_CONST: f32 = 2.85532;

/**
 * The coordinate offset of 8 minor lattices surrounding a major lattice.
 * If the major lattice is determined, we can get the lattice coordinate of a minor lattice
 * by this offset.
 */
const offset: [(i32, i32, i32); 8] = [ //offset index: 0-> x, 1 -> y, 2 -> z
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
const normal_vector: [(f32, f32, f32); 8] = [
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
const d: f32 = -3.0 / 4.0;

// voronoy analysis method for BCC lattice and cube lattice.
pub fn voronoy_ans(xyzfile: &str, output: &str) {
    let input = File::open(xyzfile).unwrap();
    let mut reader = Reader::new(input);
    // todo read atom one by one and compute its index lattice.
    let snapshot_result = reader.read_snapshot();

    match snapshot_result {
        Err(e) => {
            print!("read input xyz file error: {:?}", e);
        }
        Ok(mut snapshot) => {
            let atoms_size = snapshot.size();
            if atoms_size % 2 != 0 { // due to BCC lattice
                print!("bad atoms size");
                return;
            }
            // do analysis
            let size_dim = cube_root(atoms_size / 2);
            if size_dim == 0 { // it is a cube box
                do_analysis(size_dim, size_dim, size_dim, &mut snapshot);
            } else {
                let (size_x, size_y, size_z) = get_box_size(&snapshot.atoms);
                if size_x <= 0 || size_y <= 0 || size_z <= 0 {
                    print!("bad box size: {},{},{}", size_x, size_y, size_z);
                    return;
                }
                do_analysis(size_x as usize, size_y as usize, size_z as usize, &mut snapshot);
            }
        }
    }
}

// return the box size of simulation box to calculate 1D lattice index.
fn get_box_size(atoms: &Vec<xyzio::Atom>) -> (i32, i32, i32) {
    let mut x_min: f32 = f32::INFINITY;
    let mut y_min: f32 = f32::INFINITY;
    let mut z_min: f32 = f32::INFINITY;
    let mut x_max: f32 = f32::NEG_INFINITY;
    let mut y_max: f32 = f32::NEG_INFINITY;
    let mut z_max: f32 = f32::NEG_INFINITY;
    for i in 0..atoms.len() {
        let x = atoms[i].x;
        let y = atoms[i].y;
        let z = atoms[i].z;

        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }

        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }

        if z < z_min {
            z_min = z;
        }
        if z > z_max {
            z_max = z;
        }
    }
    let min_index = voronoy(x_min, y_min, z_min);
    let max_index = voronoy(x_max, y_max, z_max);
    return (max_index.0 - min_index.0, max_index.1 - min_index.1, max_index.2 - min_index.2);
}

fn do_analysis(x_size: usize, y_size: usize, z_size: usize, snapshot: &mut xyzio::Snapshot) {
    // scale to lattice const unit.
    let mut atoms_lat_map = BTreeMap::new();

    for i in 0..x_size * y_size * z_size {
        // calculate lattice index of each atom
        // note: x is doubled.
        let (x, y, z) = voronoy(snapshot.atoms[i].x, snapshot.atoms[i].y, snapshot.atoms[i].z);
        // z * 2 * x_size * y_size  + y * 2 * x_size  + x;
        let global_index = 2 * (x_size as i32) * (z * y_size as i32 + y) + x;

        match atoms_lat_map.get(&global_index) {
            Some(&atom_index) => {
                if atom_index == -1 {
                    // output i self only
                    println!("atom: {},{},{},{},{}", snapshot.atoms[i].element, global_index,
                             snapshot.atoms[i].x, snapshot.atoms[i].y, snapshot.atoms[i].z);
                } else {
                    // output i self and data indexed in map
                    println!("atom: {},{},{},{},{}", snapshot.atoms[atom_index as usize].element, global_index,
                             snapshot.atoms[atom_index as usize].x, snapshot.atoms[atom_index as usize].y,
                             snapshot.atoms[atom_index as usize].z);
                    println!("atom: {},{},{},{},{}", snapshot.atoms[i].element, global_index,
                             snapshot.atoms[i].x, snapshot.atoms[i].y, snapshot.atoms[i].z);
                    atoms_lat_map.insert(global_index, -1); // first already write
                }
            }
            None => {
                atoms_lat_map.entry(global_index).or_insert(i as i32);
            }
        }
    }
}

/**
 * calculate coordinate of nearest lattice for a atom:
 * X,Y,Z is the position of atom.
 */
fn voronoy(x: f32, y: f32, z: f32) -> (i32, i32, i32) {
    let lat_coord_x = (x / LATTICE_CONST).round() as i32;
    let mut lat_coord_y = (y / LATTICE_CONST).round() as i32;
    let mut lat_coord_z = (z / LATTICE_CONST).round() as i32;

    let delta_x = x / LATTICE_CONST - (lat_coord_x as f32);
    let delta_y = y / LATTICE_CONST - (lat_coord_y as f32);
    let delta_z = z / LATTICE_CONST - (lat_coord_z as f32);

    let flag_index = (if delta_z > 0.0 { 4 } else { 0 })
        | (if delta_y > 0.0 { 2 } else { 0 })
        | (if delta_x > 0.0 { 1 } else { 0 });

    let mut lat_coord_x = 2 * lat_coord_x; // x is doubled.

    if normal_vector[flag_index].0 * delta_x +
        normal_vector[flag_index].1 * delta_y +
        normal_vector[flag_index].2 * delta_z + d >= 0.0 {
        /* belongs to major lattice. */
        lat_coord_x += offset[flag_index].0;
        lat_coord_y += offset[flag_index].1;
        lat_coord_z += offset[flag_index].2;
    }
    return (lat_coord_x, lat_coord_y, lat_coord_z);
}

// return cube root of a positive integer number.
fn cube_root(n: usize) -> usize {
    let mut low = 1 as usize;
    let mut high = n;
    let mut mid = n;
    while low <= high {
        mid = (high + low) / 2;
        if n < mid * mid * mid {
            high = mid - 1;
        } else if n == mid * mid * mid {
            return mid;
        } else {
            low = mid + 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod get_box_size_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_box_size_1() {
        let mut atoms = Vec::new();
        atoms.push(xyzio::Atom {
            element: String::from("Fe"),
            x: 4.0 * LATTICE_CONST,
            y: 6.0 * LATTICE_CONST,
            z: 8.0 * LATTICE_CONST,
        });
        atoms.push(xyzio::Atom {
            element: String::from("Fe"),
            x: 2.0 * LATTICE_CONST,
            y: 1.0 * LATTICE_CONST,
            z: 9.0 * LATTICE_CONST,
        });
        // get lattice size in each dimension
        let sizes = get_box_size(&atoms);
        assert_eq!(sizes.0, 2 * 2);
        assert_eq!(sizes.1, 5);
        assert_eq!(sizes.2, 1);
    }
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

#[cfg(test)]
mod cube_root_tests {
    use super::*;

    #[test]
    fn test_cube_root() {
        assert_eq!(cube_root(1), 1);
        assert_eq!(cube_root(8), 2);
        assert_eq!(cube_root(27), 3);
        assert_eq!(cube_root(64), 4);
        assert_eq!(cube_root(65), 0);
        assert_eq!(cube_root(125), 5);
    }
}
