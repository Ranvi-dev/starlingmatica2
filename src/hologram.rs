use crate::schematic::SchematicGrid;
use crate::verifier::{evaluate_block, BlockMatchStatus};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HologramVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct HologramConfig {
    pub origin_x: i32,
    pub origin_y: i32,
    pub origin_z: i32,
    pub rotation_steps: u8,
    pub active_layer: Option<usize>,
    pub opacity: f32,
    pub visible: bool,
}

impl Default for HologramConfig {
    fn default() -> Self {
        Self {
            origin_x: 0,
            origin_y: 0,
            origin_z: 0,
            rotation_steps: 0,
            active_layer: Some(0),
            opacity: 0.45,
            visible: true,
        }
    }
}

pub fn build_hologram_mesh<F>(
    grid: &SchematicGrid,
    config: &HologramConfig,
    mut world_lookup: F,
) -> Vec<HologramVertex>
where
    F: FnMut(i32, i32, i32) -> crate::schematic::BlockState,
{
    let mut vertices = Vec::new();
    if !config.visible {
        return vertices;
    }

    let y_range = match config.active_layer {
        Some(y) if y < grid.height => y..=y,
        Some(_) => return vertices,
        None => 0..=(grid.height.saturating_sub(1)),
    };

    for y in y_range {
        for z in 0..grid.length {
            for x in 0..grid.width {
                let Some(raw_block) = grid.get_block(x, y, z) else { continue };
                if raw_block.is_air() {
                    continue;
                }

                let expected = raw_block.rotated(config.rotation_steps);
                let (rx, rz) = rotate_coords(x, z, grid.width, grid.length, config.rotation_steps);

                let wx = config.origin_x + rx as i32;
                let wy = config.origin_y + y as i32;
                let wz = config.origin_z + rz as i32;

                let actual = world_lookup(wx, wy, wz);
                let status = evaluate_block(&expected, &actual);

                let color = match status {
                    BlockMatchStatus::Correct => continue,
                    BlockMatchStatus::Missing => [0.2, 0.6, 1.0, config.opacity],
                    BlockMatchStatus::WrongBlock => [1.0, 0.2, 0.2, config.opacity],
                    BlockMatchStatus::WrongStateOrOrientation => [1.0, 0.75, 0.1, config.opacity],
                };

                push_cube_wireframe(&mut vertices, wx as f32, wy as f32, wz as f32, color);
            }
        }
    }
    vertices
}

fn rotate_coords(x: usize, z: usize, w: usize, l: usize, steps: u8) -> (usize, usize) {
    match steps % 4 {
        1 => (l.saturating_sub(1 + z), x),
        2 => (w.saturating_sub(1 + x), l.saturating_sub(1 + z)),
        3 => (z, w.saturating_sub(1 + x)),
        _ => (x, z),
    }
}

fn push_cube_wireframe(out: &mut Vec<HologramVertex>, x: f32, y: f32, z: f32, color: [f32; 4]) {
    let p = [
        [x, y, z], [x + 1.0, y, z], [x + 1.0, y, z + 1.0], [x, y, z + 1.0],
        [x, y + 1.0, z], [x + 1.0, y + 1.0, z], [x + 1.0, y + 1.0, z + 1.0], [x, y + 1.0, z + 1.0],
    ];
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    for (a, b) in edges {
        out.push(HologramVertex { position: p[a], color });
        out.push(HologramVertex { position: p[b], color });
    }
}
