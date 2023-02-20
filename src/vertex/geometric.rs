use regex::Regex;

const PATTERN: &'static str = r"v (?P<x>-*\d+\.\d+) (?P<y>-*\d+\.\d+) (?P<z>-*\d+\.\d+)";

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct GeometricVertex {
    position: [f32; 3],
}

impl Default for GeometricVertex {
    fn default() -> Self {
        Self { position: [0.0; 3] }
    }
}

impl GeometricVertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }

    pub fn from_line(line: &str) -> Result<Self, &'static str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(PATTERN).expect("Geometric vertex regex pattern invalid");
        }
        let captures = RE.captures(line);
        if captures.is_none() {
            return Err("Provided line does not define a geometric vertex.");
        }

        let captures = captures.unwrap();
        if captures.len() != 4 {
            return Err("Incorrect number of line elements");
        }

        // Extract the values
        let x: f32 = captures["x"].parse().unwrap_or_default();
        let y: f32 = captures["y"].parse().unwrap_or_default();
        let z: f32 = captures["z"].parse().unwrap_or_default();

        Ok(Self {
            position: [x, y, z],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_from_line() {
        let test_data = [
            (
                "v 1.000000 1.000000 -1.000000",
                GeometricVertex {
                    position: [1.0, 1.0, -1.0],
                },
            ),
            (
                "v -1.000000 -1.000000 1.000000",
                GeometricVertex {
                    position: [-1.0, -1.0, 1.0],
                },
            ),
            (
                "v -0.500000 1.100000 0.340000",
                GeometricVertex {
                    position: [-0.5, 1.1, 0.34],
                },
            ),
        ];

        for data in test_data {
            let line = data.0;
            let expected = data.1;

            let actual = GeometricVertex::from_line(line).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn reject_invalid_line() {
        let test_data = [
            "vn -0.0000 1.0000 -0.0000",
            "o Cube",
            "vt 0.625000 0.500000",
            "s 0",
            "# Blender 3.4.1",
            "f 5/5/1 3/3/1 1/1/1",
            "v 1.000000 1.000000",
            "v 1.s 1.000000 -1.000000",
        ];

        for line in test_data {
            let result = GeometricVertex::from_line(line);
            assert!(result.is_err());
        }
    }
}
