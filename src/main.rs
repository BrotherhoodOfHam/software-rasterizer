/*
    Simple rasterizer
*/

extern crate image;
extern crate cgmath;

mod utils;

type V2 = cgmath::Vector2<f32>;
type V3 = cgmath::Vector3<f32>;

/***********************************************************************************/

struct Painter
{
    buffer: image::RgbImage
}

impl Painter
{
    pub fn new(width: u32, height: u32) -> Painter
    {
        Painter {
            buffer: image::ImageBuffer::new(width, height)
        }
    }

    fn clamp(value: f32, max: u32) -> u32
    {
        utils::min(utils::max(0, value.floor() as i32) as u32, max - 1)
    }

    // Edge function
    fn edge(v0: &V2, v1: &V2, p: &V2) -> f32
    {
        let a = p - v0;
        let b = v1 - v0;
        (a.x * b.y) - (a.y * b.x)
    }

    // Take a position p in raster space to the triangle a->b->c and return it's corresponding 
    // barycentric coordinates iff the triangle encloses p
    fn map_triangle_pos(p: &V2, a: &V2, b: &V2, c: &V2) -> Option<V3>
    {
        let area = Self::edge(a, b, c);
        let w0 = Self::edge(a, b, p);
        let w1 = Self::edge(b, c, p);
        let w2 = Self::edge(c, a, p);
        
        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
        {
            return Some(V3{
                x: w0 / area,
                y: w1 / area,
                z: w2 / area
            })
        }

        None
    }

    // Draw triangle
    // vertices are in raster-space
    pub fn draw_triangle(&mut self, a: V2, b: V2, c: V2)
    {
        //Get bounding box of triangle
        let max = max(&a, &max(&b, &c));
        let min = min(&a, &min(&b, &c));
        //Clamp bounding box
        let xmax = Self::clamp(max.x, self.buffer.width());
        let xmin = Self::clamp(min.x, self.buffer.width());
        let ymax = Self::clamp(max.y, self.buffer.height());
        let ymin = Self::clamp(min.y, self.buffer.height());

        for y in ymin..=ymax
        {
            for x in xmin..=xmax
            {
                if let Some(coords) = Self::map_triangle_pos(
                    &V2{x: x as f32,y: y as f32},
                    &a, &b, &c)
                {
                    let pixel = image::Pixel::from_channels(
                        (200.0 * coords.x) as u8,
                        (200.0 * coords.y) as u8,
                        (200.0 * coords.z) as u8,
                        0);

                    self.buffer.put_pixel(x, y, pixel);
                }
            }
        }
    }

    pub fn save(&self, path: &str)
    {
        self.buffer.save(path).expect("unable to save file");
    }
}

fn main() {
    let mut p = Painter::new(512, 512);
    p.draw_triangle(
        V2{x: 10.0,  y: 10.0},
        V2{x: 10.0,  y: 500.0},
        V2{x: 500.0, y: 10.0}
    );
    p.save("img.png");
}

/***********************************************************************************/

fn max(a: &V2, b: &V2) -> V2
{
    V2 { x: utils::max(a.x, b.x), y: utils::max(a.y, b.y) }
}

fn min(a: &V2, b: &V2) -> V2
{
    V2 { x: utils::min(a.x, b.x), y: utils::min(a.y, b.y) }
}
