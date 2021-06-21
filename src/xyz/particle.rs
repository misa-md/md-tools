/**
 * Add support for parsing id, position and velocity in line string of xyz file.
 */

use std::iter::Iterator;

use std::str::FromStr;
use xyzio::Error;

type Real = f64;

// full information of a particle
pub struct Particle {
    pub id: u32,
    // type of atom
    pub tp: String,
    pub pos: (Real, Real, Real),
    pub v: (Real, Real, Real),
    pub f: (Real, Real, Real),
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

    fn is_velocity_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        return (self.v.0 - another.v.0).abs() < error_limit &&
            (self.v.1 - another.v.1).abs() < error_limit &&
            (self.v.2 - another.v.2).abs() < error_limit;
    }

    fn is_force_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        return (self.f.0 - another.f.0).abs() < error_limit &&
            (self.f.1 - another.f.1).abs() < error_limit &&
            (self.f.2 - another.f.2).abs() < error_limit;
    }

    // check 2 atoms' status, including position, and possible velocity, force.
    pub(crate) fn is_status_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        return self.is_pos_near_eq(another, error_limit)
            && self.is_velocity_near_eq(another, error_limit)
            && self.is_force_near_eq(another, error_limit);
    }
    // compare atoms status under periodic boundary checking
    pub(crate) fn is_status_near_eq_with_pbc(&self, another: &Particle, error_limit: f64, box_length: (f64, f64, f64)) -> bool {
        return self.is_pos_near_eq_with_pbc(another, error_limit, box_length)
            && self.is_velocity_near_eq(another, error_limit)
            && self.is_force_near_eq(another, error_limit);
    }
}

impl FromStr for Particle {
    type Err = Error;

    // the line must include id and type. while position, v and force are optional.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split_whitespace().collect();
        if splitted.len() < 2 {
            return Err(Error::IllegalState(String::from("must specific id and type")));
        }
        let mut pos: (Real, Real, Real) = (0.0, 0.0, 0.0);
        let mut v: (Real, Real, Real) = (0.0, 0.0, 0.0);
        let mut f: (Real, Real, Real) = (0.0, 0.0, 0.0);

        if splitted.len() < 2 + 3 {
            pos = (splitted[2].parse::<Real>().unwrap(), splitted[3].parse::<Real>().unwrap(), splitted[4].parse::<Real>().unwrap())
        }
        if splitted.len() < 2 + 3 + 3 {
            v = (splitted[5].parse::<Real>().unwrap(), splitted[6].parse::<Real>().unwrap(), splitted[7].parse::<Real>().unwrap())
        }
        if splitted.len() < 2 + 3 + 3 + 3 {
            f = (splitted[8].parse::<Real>().unwrap(), splitted[9].parse::<Real>().unwrap(), splitted[10].parse::<Real>().unwrap())
        }
        Ok(Particle {
            id: splitted[0].parse::<Real>()? as u32,
            tp: splitted[1].parse().unwrap(),
            pos,
            v,
            f,
        })
    }
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("id: {}, position: ({}, {}, {}), v:({}, {}, {})",
                self.id, self.pos.0, self.pos.1, self.pos.1, self.v.0, self.v.1, self.v.2)
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
