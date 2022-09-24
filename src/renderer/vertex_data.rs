use bevy::{
    math::{vec2, vec3, UVec2, Vec2, Vec3},
    prelude::Component,
};
use sark_grids::{point::Point2d, GridPoint, Size2d};

//     pub fn terminal_resize(
//         &mut self,
//         origin: impl Point2d,
//         term_size: impl Size2d,
//         tile_size: Vec2,
//     ) {
//         let len = term_size.len();
//         let width = term_size.width();

//         self.clear();
//         self.reserve(len);

//         let mut helper = VertHelper {
//             origin: origin.as_vec2(),
//             tile_size,
//             data: self,
//         };

//         for i in 0..len {
//             let x = i % width;
//             let y = i / width;

//             helper.tile_at([x, y]);
//         }
//     }

//     pub fn border_resize(&mut self, origin: impl Point2d, term_size: UVec2, tile_size: Vec2) {
//         let width = term_size.width() + 2;
//         let height = term_size.height() + 2;
//         let len = (width * 2) + ((height - 2) * 2);
//         let origin = origin.as_vec2();

//         self.verts.clear();
//         self.verts
//             .reserve((len * 4).saturating_sub(self.verts.capacity()));

//         self.indices.clear();
//         self.indices
//             .reserve((len * 6).saturating_sub(self.indices.capacity()));

//         let top = height - 1;
//         let bottom = 0;
//         let left = 0;
//         let right = width - 1;

//         let mut helper = VertHelper {
//             origin,
//             tile_size,
//             data: self,
//         };

//         helper.tile_at([left, bottom]);
//         helper.tile_at([left, top]);
//         helper.tile_at([right, top]);
//         helper.tile_at([right, bottom]);

//         for x in 1..width - 1 {
//             helper.tile_at([x, bottom]);
//             helper.tile_at([x, top]);
//         }

//         for y in 1..height - 1 {
//             helper.tile_at([left, y]);
//             helper.tile_at([right, y]);
//         }
//     }
// }
