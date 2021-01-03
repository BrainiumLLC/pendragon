use crate::tess;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: gee::Point<f32>,
    pub tex_coord: gee::Point<f32>,
    pub normal: gee::Vector<f32>,
}

pub struct FillVertexConstructor {
    // cached version of gee::Rect in the format we want
    pub top_left: gee::Point<f32>,
    pub scale: gee::Vector<f32>,
}

impl FillVertexConstructor {
    pub fn new(bounding_box: gee::Rect<f32>) -> Self {
        Self {
            top_left: bounding_box.top_left(),
            scale: bounding_box.size().to_vector().map(|x| x.recip()),
        }
    }
}

impl tess::VertexConstructor<tess::FillVertex, Vertex> for FillVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::FillVertex) -> Vertex {
        let pos = gee::Point::new(vertex.position.x, vertex.position.y);
        let rel_coord = pos - self.top_left;
        let tex_coord = gee::Point::new(rel_coord.dx * self.scale.dx, rel_coord.dy * self.scale.dy);
        let normal = gee::Vector::new(vertex.normal.x, vertex.normal.y);
        Vertex {
            pos,
            tex_coord,
            normal,
        }
    }
}

pub struct StrokeVertexConstructor {
    pub stroke_width: f32,
    pub texture_aspect_ratio: f32,
}

impl StrokeVertexConstructor {
    pub fn new(stroke_width: f32, texture_aspect_ratio: f32) -> Self {
        Self {
            stroke_width,
            texture_aspect_ratio,
        }
    }
}

impl tess::VertexConstructor<tess::StrokeVertex, Vertex> for StrokeVertexConstructor {
    fn new_vertex(&mut self, vertex: tess::StrokeVertex) -> Vertex {
        let pos = gee::Point::new(vertex.position.x, vertex.position.y);
        let x = match vertex.side {
            tess::Side::Left => 1.0,
            tess::Side::Right => 0.0,
        };
        let y = vertex.advancement / self.stroke_width * self.texture_aspect_ratio;
        let tex_coord = gee::Point::new(x, y);
        let normal = gee::Vector::new(vertex.normal.x, vertex.normal.y);
        Vertex {
            pos,
            tex_coord,
            normal,
        }
    }
}
