/*
    Simple rasterizer
*/

extern crate image;
extern crate cgmath;

mod utils;
mod model;

use image::*;
use cgmath::*;

/***********************************************************************************/

struct Painter
{
    frame_buffer: RgbImage,
    depth_buffer: Box<[f32]>
}

impl Painter
{
    pub fn new(width: u32, height: u32) -> Painter
    {
        let mut v: Vec<f32> = Vec::new();
        v.resize((width * height) as usize, std::f32::MAX);

        Painter {
            frame_buffer: image::ImageBuffer::new(width, height),
            depth_buffer: v.into_boxed_slice()
        }
    }

    fn clamp(value: f32, max: u32) -> u32
    {
        utils::min(utils::max(0, value.floor() as i32) as u32, max - 1)
    }

    // Edge function
    fn edge(v0: &Vector2<f32>, v1: &Vector2<f32>, p: &Vector2<f32>) -> f32
    {
        let a = p - v0;
        let b = v1 - v0;
        (a.x * b.y) - (a.y * b.x)
    }

    // Take a position p in raster space to the triangle a->b->c and return it's corresponding 
    // barycentric coordinates iff the triangle encloses p
    fn map_triangle_pos(p: &Vector2<f32>, a: &Vector2<f32>, b: &Vector2<f32>, c: &Vector2<f32>) -> Option<Vector3<f32>>
    {
        // The edge function gives 2 * the area of the triangle formed by the 3 given vectors
        let area = Self::edge(a, b, c);
        // The result of the edge function is also used to find the barycentric coordinates
        let w0 = Self::edge(b, c, p);
        let w1 = Self::edge(c, a, p);
        let w2 = Self::edge(a, b, p);

        //If point is inside triangle
        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
        {
            //Normalize barycentric coordinates
            return Some(Vector3::new(
                w0 / area,
                w1 / area,
                w2 / area
            ))
        }

        None
    }

    // Draw triangle a->b->c
    // vertex coordinates are in raster-space
    pub fn draw_triangle(&mut self, a: &model::Vertex, b: &model::Vertex, c: &model::Vertex)
    {
        //Get bounding box of triangle
        let max = max(&a.pos, &max(&b.pos, &c.pos));
        let min = min(&a.pos, &min(&b.pos, &c.pos));
        //Clamp bounding box
        let xmax = Self::clamp(max.x, self.frame_buffer.width());
        let xmin = Self::clamp(min.x, self.frame_buffer.width());
        let ymax = Self::clamp(max.y, self.frame_buffer.height());
        let ymin = Self::clamp(min.y, self.frame_buffer.height());

        for y in ymin..=ymax
        {
            for x in xmin..=xmax
            {
                let idx = ((y * self.frame_buffer.width()) + x) as usize;

                let centre = Vector2::new(0.5, 0.5);
                let p = Vector2::new(x as f32, y as f32) + centre;

                if let Some(bcoords) = Self::map_triangle_pos(
                    &p,
                    &xy(&a.pos), &xy(&b.pos), &xy(&c.pos))
                {
                    let vtx = model::Vertex::interpolate(&a, &b, &c, &bcoords);
                    
                    if vtx.pos.z < self.depth_buffer[idx]
                    {
                        self.depth_buffer[idx] = vtx.pos.z;
                        
                        let pixel = image::Pixel::from_channels(
                            (255.0 * vtx.colour.x) as u8,
                            (255.0 * vtx.colour.y) as u8,
                            (255.0 * vtx.colour.z) as u8,
                            0);
                        self.frame_buffer.put_pixel(x, y, pixel);
                    }
                }
            }
        }
    }

    // Apply viewport transformation
    // Converts ND [-1,1] coordinates to raster coordinates [0,w]
    // z component is not converted
    fn to_raster_space(&self, vtx: &model::Vertex) -> model::Vertex
    {
        let vector = vtx.pos;
        let d = Vector2::new(self.frame_buffer.width() as f32, self.frame_buffer.height() as f32);
        
        let mut new_vtx = vtx.clone();
        new_vtx.pos = Vector3::new(((vector.x + 1.0) / 2.0) * d.x, ((vector.y + 1.0) / 2.0) * d.y, vector.z);
        new_vtx
    }

    pub fn draw(&mut self, model: &model::Model)
    {
        //For each triangle
        for i in (0..model.len()).step_by(3)
        {
            let a = &model[i + 0];
            let b = &model[i + 1];
            let c = &model[i + 2];

            self.draw_triangle(
                &self.to_raster_space(&a),
                &self.to_raster_space(&b),
                &self.to_raster_space(&c)
            );
        }
    }

    pub fn save(&self, path: &str)
    {
        self.frame_buffer.save(path).expect("unable to save file");
        /*
        let v: Vec<u8> = self.depth_buffer.to_vec().into_iter().map(
            |x| (x * 255.0) as u8
        ).collect();
        
        image::save_buffer(
            path, v.as_slice(),
            self.frame_buffer.width(), self.frame_buffer.height(),
            image::ColorType::Gray(1)
        ).unwrap();
        // */
    }
}


fn main() {
    let mut p = Painter::new(512, 512);
    let view = Matrix4::look_at(Point3::new(1.5, -3.0, -2.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    let proj = perspective(Deg(90.0), 1.0, 0.01, 10.0);

    let mut model = model::get_cube_model();
    model::tranform_model(&mut model, view);
    model::project_model(&mut model, proj);

    p.draw(&model);

    p.save("img.png");
}

/***********************************************************************************/

fn xy(v: &Vector3<f32>) -> Vector2<f32>
{
    Vector2::new(v.x, v.y)
}

fn max(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32>
{
    Vector3::new(utils::max(a.x, b.x), utils::max(a.y, b.y), utils::max(a.z, b.z))
}

fn min(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32>
{
    Vector3::new(utils::min(a.x, b.x), utils::min(a.y, b.y), utils::min(a.z, b.z))
}
