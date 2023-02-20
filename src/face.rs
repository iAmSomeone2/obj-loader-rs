use crate::vertex::{geometric::GeometricVertex, normal::VertexNormal, texture::TextureVertex};
use regex::Regex;

// const BASIC_FACE_PATTERN: &'static str = r"f (?P<v0>-*\d+) (?P<v1>-*\d+) (?P<v2>-*\d+)";
const PATTERN: &'static str = r"f (?P<v0>-*\d+)/?(?P<t0>-*\d+)?/?(?P<n0>-*\d+)? (?P<v1>-*\d+)/?(?P<t1>-*\d+)?/?(?P<n1>-*\d+)? (?P<v2>-*\d+)/?(?P<t2>-*\d+)?/?(?P<n2>-*\d+)?";

#[derive(Debug)]
pub struct Face {
    vertices: [GeometricVertex; 3],
    texture_coordinates: Option<[TextureVertex; 3]>,
    normals: Option<[VertexNormal; 3]>,
}

impl Default for Face {
    fn default() -> Self {
        Self {
            vertices: [GeometricVertex::default(); 3],
            texture_coordinates: None,
            normals: None,
        }
    }
}

impl Face {
    fn set_vertices_by_index(&mut self, indices: (i32, i32, i32), vertices: &Vec<GeometricVertex>) {
        let v0 = if indices.0 > 0 {
            (indices.0 - 1) as usize
        } else {
            vertices.len() + (indices.0 as usize)
        };

        let v1 = if indices.1 > 0 {
            (indices.1 - 1) as usize
        } else {
            vertices.len() + (indices.1 as usize)
        };

        let v2 = if indices.2 > 0 {
            (indices.2 - 1) as usize
        } else {
            vertices.len() + (indices.2 as usize)
        };

        self.vertices[0] = vertices[v0].clone();
        self.vertices[1] = vertices[v1].clone();
        self.vertices[2] = vertices[v2].clone();
    }

    fn set_texture_coords_by_index(
        &mut self,
        indices: (i32, i32, i32),
        tex_coords: &Vec<TextureVertex>,
    ) {
        let t0 = if indices.0 > 0 {
            (indices.0 - 1) as usize
        } else {
            tex_coords.len() + (indices.0 as usize)
        };

        let t1 = if indices.1 > 0 {
            (indices.1 - 1) as usize
        } else {
            tex_coords.len() + (indices.1 as usize)
        };

        let t2 = if indices.2 > 0 {
            (indices.2 - 1) as usize
        } else {
            tex_coords.len() + (indices.2 as usize)
        };

        if self.texture_coordinates.is_none() {
            self.texture_coordinates = Some([TextureVertex::default(); 3]);
        }

        if let Some(ref mut texture_coordinates) = self.texture_coordinates {
            texture_coordinates[0] = tex_coords[t0].clone();
            texture_coordinates[1] = tex_coords[t1].clone();
            texture_coordinates[2] = tex_coords[t2].clone();
        }
    }

    fn set_normals_by_index(&mut self, indices: (i32, i32, i32), normals: &Vec<VertexNormal>) {
        let n0 = if indices.0 > 0 {
            (indices.0 - 1) as usize
        } else {
            normals.len() + (indices.0 as usize)
        };

        let n1 = if indices.1 > 0 {
            (indices.1 - 1) as usize
        } else {
            normals.len() + (indices.1 as usize)
        };

        let n2 = if indices.2 > 0 {
            (indices.2 - 1) as usize
        } else {
            normals.len() + (indices.2 as usize)
        };

        if self.normals.is_none() {
            self.normals = Some([VertexNormal::default(); 3]);
        }

        if let Some(ref mut norms) = self.normals {
            norms[0] = normals[n0].clone();
            norms[1] = normals[n1].clone();
            norms[2] = normals[n2].clone();
        }
    }

    pub fn from_line(
        line: &str,
        vertices: &Vec<GeometricVertex>,
        tex_coords: Option<&Vec<TextureVertex>>,
        normals: Option<&Vec<VertexNormal>>,
    ) -> Result<Self, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(PATTERN).expect("Face regex pattern invalid");
        }

        let captures = RE.captures(line);
        if captures.is_none() {
            return Err("Provided line does not define a face.");
        }

        let captures = captures.unwrap();
        if captures.len() < 4 {
            return Err("Incorrect number of elements in line");
        }

        let mut face = Face::default();

        // Get vertex indices first. They're guaranteed to be defined
        let v0: i32 = captures["v0"].parse().unwrap_or(0);
        let v1: i32 = captures["v1"].parse().unwrap_or(0);
        let v2: i32 = captures["v2"].parse().unwrap_or(0);
        face.set_vertices_by_index((v0, v1, v2), vertices);

        // Handle texture coordinates
        if let Some(tex_coords) = tex_coords {
            let t0: i32 = captures["t0"].parse().unwrap_or(0);
            let t1: i32 = captures["t1"].parse().unwrap_or(0);
            let t2: i32 = captures["t2"].parse().unwrap_or(0);
            face.set_texture_coords_by_index((t0, t1, t2), tex_coords);
        }

        // Handle normals
        if let Some(norms) = normals {
            let n0: i32 = captures["n0"].parse().unwrap_or(0);
            let n1: i32 = captures["n1"].parse().unwrap_or(0);
            let n2: i32 = captures["n2"].parse().unwrap_or(0);
            face.set_normals_by_index((n0, n1, n2), norms);
        }

        Ok(face)
    }
}

#[cfg(test)]
mod test {
    use crate::vertex::{geometric::GeometricVertex, normal::VertexNormal, texture::TextureVertex};

    use super::Face;

    const TEST_VERTS: [&'static str; 8] = [
        "v 1.000000 1.000000 -1.000000",
        "v 1.000000 -1.000000 -1.000000",
        "v 1.000000 1.000000 1.000000",
        "v 1.000000 -1.000000 1.000000",
        "v -1.000000 1.000000 -1.000000",
        "v -1.000000 -1.000000 -1.000000",
        "v -1.000000 1.000000 1.000000",
        "v -1.000000 -1.000000 1.000000",
    ];

    const TEST_NORMS: [&'static str; 6] = [
        "vn -0.0000 1.0000 -0.0000",
        "vn -0.0000 -0.0000 1.0000",
        "vn -1.0000 -0.0000 -0.0000",
        "vn -0.0000 -1.0000 -0.0000",
        "vn 1.0000 -0.0000 -0.0000",
        "vn -0.0000 -0.0000 -1.0000",
    ];

    const TEST_TEXT_COORDS: [&'static str; 14] = [
        "vt 0.625000 0.500000",
        "vt 0.375000 0.500000",
        "vt 0.625000 0.750000",
        "vt 0.375000 0.750000",
        "vt 0.875000 0.500000",
        "vt 0.625000 0.250000",
        "vt 0.125000 0.500000",
        "vt 0.375000 0.250000",
        "vt 0.875000 0.750000",
        "vt 0.625000 1.000000",
        "vt 0.625000 0.000000",
        "vt 0.375000 0.000000",
        "vt 0.375000 1.000000",
        "vt 0.125000 0.750000",
    ];

    const FACE_DATA: [&'static str; 12] = [
        "f 5/5/1 3/3/1 1/1/1",
        "f 3/3/2 8/13/2 4/4/2",
        "f 7/11/3 6/8/3 8/12/3",
        "f 2/2/4 8/14/4 6/7/4",
        "f 1/1/5 4/4/5 2/2/5",
        "f 5/6/6 2/2/6 6/8/6",
        "f 5/5/1 7/9/1 3/3/1",
        "f 3/3/2 7/10/2 8/13/2",
        "f 7/11/3 5/6/3 6/8/3",
        "f 2/2/4 4/4/4 8/14/4",
        "f 1/1/5 3/3/5 4/4/5",
        "f 5/6/6 1/1/6 2/2/6",
    ];

    fn create_geometric_verts() -> Vec<GeometricVertex> {
        let mut verts = Vec::with_capacity(TEST_VERTS.len());

        for line in TEST_VERTS {
            let vert = GeometricVertex::from_line(line).unwrap();
            verts.push(vert);
        }

        verts
    }

    fn create_normal_verts() -> Vec<VertexNormal> {
        let mut verts = Vec::with_capacity(TEST_NORMS.len());

        for line in TEST_NORMS {
            let vert = VertexNormal::from_line(line).unwrap();
            verts.push(vert);
        }

        verts
    }

    fn create_texture_verts() -> Vec<TextureVertex> {
        let mut verts = Vec::with_capacity(TEST_TEXT_COORDS.len());

        for line in TEST_TEXT_COORDS {
            let vert = TextureVertex::from_line(line).unwrap();
            verts.push(vert);
        }

        verts
    }

    #[test]
    fn create_from_line() {
        let verts = create_geometric_verts();
        let norms = create_normal_verts();
        let text_verts = create_texture_verts();

        let face = Face::from_line(FACE_DATA[0], &verts, Some(&text_verts), Some(&norms)).unwrap();
        println!("{:?}", face);
    }
}
