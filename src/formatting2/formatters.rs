use bevy::prelude::Color;

pub trait TileFormatter {
    fn apply(&self, 
        fg_colors: &mut [Color],
        bg_colors: &mut [Color],
        verts: &mut [[f32;3]],
        uvs: &mut [[f32;2]],
         
    );
}

pub struct FGColorFormatter(pub Color);

impl TileFormatter for FGColorFormatter {
    fn apply(&self, 
        fg_colors: &mut [Color],
        _bg_colors: &mut [Color],
        _verts: &mut [[f32;3]],
        _uvs: &mut [[f32;2]],
         
    ) {
        fg_colors[..].iter_mut().for_each(|c| *c = self.0);
    }
}

pub struct BGColorFormatter(pub Color);

impl TileFormatter for BGColorFormatter {
    fn apply(&self, 
        _fg_colors: &mut [Color],
        bg_colors: &mut [Color],
        _verts: &mut [[f32;3]],
        _uvs: &mut [[f32;2]],
         
    ) {
        bg_colors[..].iter_mut().for_each(|c| *c = self.0);
    }
}

pub struct Invert();

impl TileFormatter for Invert {
    fn apply(&self, 
        fg_colors: &mut [Color],
        bg_colors: &mut [Color],
        _verts: &mut [[f32;3]],
        _uvs: &mut [[f32;2]],
         
    ) {
        // Doesn't work?
        //std::mem::swap(&mut fg_colors,&mut bg_colors);
        let mut buffer;
        for (a,b) in fg_colors.iter_mut().zip(bg_colors.iter_mut()) {
            buffer = *a;
            *a = *b;
            *b = buffer;
        }
    }
}

pub struct FlipHorizontal();

impl TileFormatter for FlipHorizontal {
    fn apply(&self, 
        _fg_colors: &mut [Color],
        _bg_colors: &mut [Color],
        _verts: &mut [[f32;3]],
        uvs: &mut [[f32;2]],
         
    ) {
        for i in (0..uvs.len()).step_by(4) {
            uvs.swap(i,i+2);
            uvs.swap(i+1,i+3)
        }
    }
}

pub struct Offset(pub [f32;2]);

impl TileFormatter for Offset {
    fn apply(&self, 
        _fg_colors: &mut [Color],
        _bg_colors: &mut [Color],
        verts: &mut [[f32;3]],
        _uvs: &mut [[f32;2]],
         
    ) {
        for chunk in verts.chunks_mut(4) {
            for vert in chunk {
                vert[0] += self.0[0];
                vert[1] += self.0[1];
            }
        }
    }
}

pub struct Jumble();

impl TileFormatter for Jumble {
    fn apply(&self, 
        _fg_colors: &mut [Color],
        _bg_colors: &mut [Color],
        verts: &mut [[f32;3]],
        _uvs: &mut [[f32;2]],
         
    ) {
        for (i, chunk) in verts.chunks_mut(4).enumerate() {
            let offset = match i % 2 == 0 {
                true => 0.5,
                false => -0.5
            };
            for vert in chunk {
                vert[0] += offset;
                vert[1] += offset;
            }
        }
    }
}
