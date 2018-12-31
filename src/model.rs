use std;
use cgmath::*;

#[derive(Debug, Clone)]
pub struct Vertex
{
    pub pos: Vector3<f32>,
    pub colour: Vector3<f32>
}

impl Vertex
{
    pub fn interpolate(a: &Vertex, b: &Vertex, c: &Vertex, weights: &Vector3<f32>) -> Vertex
    {
        Vertex {
            pos: (a.pos * weights.x) + (b.pos * weights.y) + (c.pos * weights.z),
            colour: (a.colour * weights.x) + (b.colour * weights.y) + (c.colour * weights.z)
        }
    }
}

pub type Model = Vec<Vertex>;

pub fn create_model(positions: &[(f32,f32,f32)], colours: &[(f32,f32,f32)], indices: &[usize]) -> Model
{
    assert!(positions.len() == colours.len());

    let mut vts = Vec::new();
    for i in indices
    {
        let (x,y,z) = positions[*i];
        let (cx, cy, cz) = colours[*i];
        vts.push(Vertex{
            pos: Vector3::new(x, y, z),
            colour: Vector3::new(cx, cy, cz)
        });
    }
    vts
}

pub fn get_cube_model() -> Model
{
    let vertices = [
        //front
        (-1.0, -1.0,  -1.0),
        (1.0, -1.0,  -1.0),
        (1.0,  1.0,  -1.0),
        (-1.0,  1.0,  -1.0),
        // back
        (-1.0, -1.0, 1.0),
        (1.0, -1.0, 1.0),
        (1.0,  1.0, 1.0),
        (-1.0,  1.0, 1.0)
    ];

    let vertex_colours = [
        // front colors
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 1.0, 1.0),
        // back colors
        (1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 1.0, 1.0)
    ];

    let indices = [
        // front
		0, 1, 2,
		2, 3, 0,
		// right
		1, 5, 6,
		6, 2, 1,
		// back
		7, 6, 5,
		5, 4, 7,
		// left
		4, 0, 3,
		3, 7, 4,
		// bottom
		4, 5, 1,
		1, 0, 4,
		// top
		3, 2, 6,
		6, 7, 3
    ];

    create_model(&vertices, &vertex_colours, &indices)
}

pub fn tranform_model(model: &mut Model, matrix: Matrix4<f32>)
{
    for vtx in model.iter_mut()
    {
        let mut v = Vector4::new(vtx.pos.x, vtx.pos.y, vtx.pos.z, 1.0);
        v = matrix * v;
        vtx.pos.x = v.x;
        vtx.pos.y = v.y;
        vtx.pos.z = v.z;
    }
}

pub fn project_model(model: &mut Model, matrix: Matrix4<f32>)
{
    for vtx in model.iter_mut()
    {
        let mut v = Vector4::new(vtx.pos.x, vtx.pos.y, vtx.pos.z, 1.0);
        v = matrix * v;
        vtx.pos.x = v.x;
        vtx.pos.y = v.y;
        vtx.pos.z = v.z;
        vtx.pos = vtx.pos / v.w;
    }
}