/**
 * Add support for parsing id, position and velocity in line string of xyz file.
 */

use std::iter::Iterator;

use std::str::FromStr;
use xyzio::Error;

type Real = f64;

// full information of a particle (e.g. position and velocity).
pub struct Particle {
    pub id: u32,
    // type of atom
    pub tp: String,
    pub pos: (Real, Real, Real),
    pub extra_data: Vec<Real>,
}

impl Particle {
    // check 2 atoms' position with an error limit
    pub(crate) fn is_pos_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        return (self.pos.0 - another.pos.0).abs() < error_limit &&
            (self.pos.1 - another.pos.1).abs() < error_limit &&
            (self.pos.2 - another.pos.2).abs() < error_limit;
    }

    // check 2 atoms' position with an error limit and periodic boundary condition.
    pub(crate) fn is_pos_near_eq_with_pbc(&self, another: &Particle, error_limit: f64, (box_x, box_y, box_z): (f64, f64, f64)) -> bool {
        return ((self.pos.0 - another.pos.0).abs() < error_limit || ((self.pos.0 - another.pos.0).abs() - box_x).abs() < error_limit)
            && ((self.pos.1 - another.pos.1).abs() < error_limit || ((self.pos.1 - another.pos.1).abs() - box_y).abs() < error_limit)
            && ((self.pos.2 - another.pos.2).abs() < error_limit || ((self.pos.2 - another.pos.2).abs() - box_z).abs() < error_limit);
    }

    fn compare_extra_data(&self, another: &Particle, error_limit: f64) -> bool {
        if self.extra_data.len() != another.extra_data.len() {
            return false;
        }
        let mut i = 0;
        for real_value in &self.extra_data {
            if (real_value - another.extra_data[i]).abs() >= error_limit {
                return false;
            }
            i += 1;
        }
        return true;
    }

    // check 2 atoms' status, including position, and possible velocity, force.
    pub(crate) fn is_status_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        return self.is_pos_near_eq(another, error_limit) && self.compare_extra_data(another, error_limit);
    }

    // compare atoms status under periodic boundary checking
    pub(crate) fn is_status_near_eq_with_pbc(&self, another: &Particle, error_limit: f64, box_length: (f64, f64, f64)) -> bool {
        return self.is_pos_near_eq_with_pbc(another, error_limit, box_length)
            && self.compare_extra_data(another, error_limit);
    }
}

impl FromStr for Particle {
    type Err = Error;

    // the line must include id and type and position. while velocity, force and other data are optional.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        if split.len() < 5 || split.len() % 3 != 2 {
            return Err(Error::IllegalState(String::from("must specific id, type and position")));
        }

        let mut pos: (Real, Real, Real) = (
            split[2].parse::<Real>().unwrap(),
            split[3].parse::<Real>().unwrap(),
            split[4].parse::<Real>().unwrap(),
        );

        let mut particle = Particle {
            id: split[0].parse::<Real>()? as u32,
            tp: split[1].parse().unwrap(),
            pos,
            extra_data: vec![],
        };
        let mut i = 0;
        for real_str in split {
            if i >= 5 {
                particle.extra_data.push(real_str.parse::<Real>()?);
            }
            i += 1;
        }
        Ok(particle)
    }
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("id: {}, position: ({}, {}, {}), extra_data: {:?})",
                self.id, self.pos.0, self.pos.1, self.pos.2, self.extra_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xyz::xyz_reader::Reader;

    #[test]
    fn test_reader() {
        let data: &[u8] = b"\
            3
            commnet
            1 Fe 1.0 2.0 3.0 1.0 2.0 3.0
            2 Fe 4.0 3.0 6.0 4.0 3.0 6.0
            3 Fe 5.0 1.5 4.0 5.0 1.5 4.0";
        let mut reader = Reader::new(data);
        let success = reader.read_snapshot::<Particle>();
        assert!(success.is_ok());

        let snapshot = success.unwrap();
        assert_eq!(3, snapshot.size());
        assert_eq!(3, snapshot.atoms[2].id);
    }
}
