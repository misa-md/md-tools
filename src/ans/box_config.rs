use crate::ans::voronoy;

/**
 * Created by genshen at 2020/12/25
 */

pub struct BoxConfig {
    pub box_size: Vec<u64>,
    // from input, length can be 0
    pub box_size_: (usize, usize, usize), // box size after determination
}

// set simulation box config and return status: ture for setting ok, false for not ok.
pub fn config_simulation_box(snapshot: &xyzio::Snapshot, box_config: &mut BoxConfig, verbose: bool) -> bool {
    let atoms_size = snapshot.size();
    if atoms_size % 2 != 0 { // due to feature of BCC lattice
        println!("bad atoms size");
        return false;
    }

    // determine box size
    if box_config.box_size.len() == 0 {
        // automatically determine calculate box size via atoms position
        let size_dim = cube_root(atoms_size / 2);
        if size_dim != 0 { // it is a cube box
            box_config.box_size_ = (size_dim, size_dim, size_dim);
        } else {
            box_config.box_size_ = auto_get_box_size(&snapshot.atoms);
        }
    } else {
        box_config.box_size_ = (box_config.box_size[0] as usize, box_config.box_size[1] as usize, box_config.box_size[2] as usize);
    }

    // check box size
    let (box_size_x, box_size_y, box_size_z) = box_config.box_size_;
    if box_size_x == 0 || box_size_y == 0 || box_size_z == 0 {
        println!("bad box size");
        return false;
    }
    if box_size_x * box_size_y * box_size_z != atoms_size {
        println!("Warning: box size ({},{},{}) not match atoms size.", box_size_x, box_size_y, box_size_z);
        return false;
    }
    if verbose {
        println!("box size: [{},{},{}]", box_size_x, box_size_y, box_size_z);
    }
    return true;
}

// By passing the atoms position list,
// then we can get the box size of simulation box, which can be used to calculating 1D lattice index.
// If the box size in some dimension is not as desired, 0 will be return in the dimension.
fn auto_get_box_size(atoms: &Vec<xyzio::Atom>) -> (usize, usize, usize) {
    let mut x_min = f32::INFINITY; // todo can use f64 as float
    let mut y_min = f32::INFINITY;
    let mut z_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    let mut y_max = f32::NEG_INFINITY;
    let mut z_max = f32::NEG_INFINITY;
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

    let (size_x_, size_y_, size_z_) = voronoy::voronoy(x_max - x_min, y_max - y_min, z_max - z_min);
    let mut sizes = (0, 0, 0);

    if size_x_ < 0 {
        sizes.0 = 0;
    } else {
        sizes.0 = size_x_ as usize;
    }
    if size_y_ < 0 {
        sizes.1 = 0;
    } else {
        sizes.1 = size_y_ as usize;
    }

    if size_z_ < 0 {
        sizes.2 = 0;
    } else {
        sizes.2 = size_z_ as usize;
    }
    return sizes;
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
            x: 4.0 * voronoy::LATTICE_CONST,
            y: 6.0 * voronoy::LATTICE_CONST,
            z: 8.0 * voronoy::LATTICE_CONST,
        });
        atoms.push(xyzio::Atom {
            element: String::from("Fe"),
            x: 2.0 * voronoy::LATTICE_CONST,
            y: 1.0 * voronoy::LATTICE_CONST,
            z: 9.0 * voronoy::LATTICE_CONST,
        });
        // get lattice size in each dimension
        let sizes = auto_get_box_size(&atoms);
        assert_eq!(sizes.0, 2 * 2);
        assert_eq!(sizes.1, 5);
        assert_eq!(sizes.2, 1);
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
