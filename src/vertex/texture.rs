use regex::Regex;

const PATTERN: &'static str = r"vt (?P<u>-*\d+\.\d+) (?P<v>-*\d+\.\d+)";

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TextureVertex {
    coordinate: [f32; 2],
}

impl Default for TextureVertex {
    fn default() -> Self {
        Self {
            coordinate: [0.0; 2],
        }
    }
}

impl TextureVertex {
    pub fn from_line(line: &str) -> Result<Self, &'static str> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(PATTERN).expect("Texture vertex regex pattern invalid");
        }
        let captures = RE.captures(line);
        if captures.is_none() {
            return Err("Provided line does not define a texture vertex.");
        }

        let captures = captures.unwrap();
        if captures.len() != 3 {
            return Err("Incorrect number of line elements");
        }

        // Extract the values
        let u: f32 = captures["u"].parse().unwrap_or_default();
        if u < 0.0 {
            return Err("Negative 'u' value in texture coordinate");
        }
        let v: f32 = captures["v"].parse().unwrap_or_default();
        if v < 0.0 {
            return Err("Negative 'v' value in texture coordinate");
        }

        Ok(Self { coordinate: [u, v] })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_from_line() {
        let test_data = [
            (
                "vt 1.000000 1.000000",
                TextureVertex {
                    coordinate: [1.0, 1.0],
                },
            ),
            (
                "vt 0.0000 0.00000",
                TextureVertex {
                    coordinate: [0.0, 0.0],
                },
            ),
            (
                "vt 0.5600 0.340000",
                TextureVertex {
                    coordinate: [0.56, 0.34],
                },
            ),
        ];

        for data in test_data {
            let line = data.0;
            let expected = data.1;

            let actual = TextureVertex::from_line(line).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn reject_invalid_line() {
        let test_data = [
            "v -0.0000 1.0000 -0.0000",
            "o Cube",
            "vt -0.625000 0.500000",
            "vt 0.625000 -0.500000",
            "s 0",
            "# Blender 3.4.1",
            "f 5/5/1 3/3/1 1/1/1",
            "vn 1.000000 1.000000",
            "vn 1.s 1.000000 -1.000000",
            "vt",
        ];

        for line in test_data {
            let result = TextureVertex::from_line(line);
            assert!(result.is_err());
        }
    }
}
