use regex::Regex;

const PATTERN: &'static str = r"vn (?P<x>-*\d+\.\d+) (?P<y>-*\d+\.\d+) (?P<z>-*\d+\.\d+)";

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct VertexNormal {
    direction: [f32; 3],
}

impl Default for VertexNormal {
    fn default() -> Self {
        Self {
            direction: [0.0; 3],
        }
    }
}

impl VertexNormal {
    pub fn from_line(line: &str) -> Result<Self, &'static str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(PATTERN).expect("Vertex normal regex pattern invalid");
        }
        let captures = RE.captures(line);
        if captures.is_none() {
            return Err("Provided line does not define a vertex normal.");
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
            direction: [x, y, z],
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
                "vn 1.0000 1.0000 -1.0000",
                VertexNormal {
                    direction: [1.0, 1.0, -1.0],
                },
            ),
            (
                "vn -1.000000 -1.000000 1.000000",
                VertexNormal {
                    direction: [-1.0, -1.0, 1.0],
                },
            ),
            (
                "vn -0.500000 1.100000 0.340000",
                VertexNormal {
                    direction: [-0.5, 1.1, 0.34],
                },
            ),
        ];

        for data in test_data {
            let line = data.0;
            let expected = data.1;

            let actual = VertexNormal::from_line(line).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn reject_invalid_line() {
        let test_data = [
            "v -0.0000 1.0000 -0.0000",
            "o Cube",
            "vt 0.625000 0.500000",
            "s 0",
            "# Blender 3.4.1",
            "f 5/5/1 3/3/1 1/1/1",
            "vn 1.000000 1.000000",
            "vn 1.s 1.000000 -1.000000",
        ];

        for line in test_data {
            let result = VertexNormal::from_line(line);
            assert!(result.is_err());
        }
    }
}
