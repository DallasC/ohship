use crate::GameAssets;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::Background,
    lifecycle::{Asset, Window},
    load_file, Future, Result,
};
use std::path::Path;

pub struct Tile {
    shape: Rectangle,
    texture: String,
    rotation: f32,
    z: i8,
    travel: bool,
}

impl Tile {
    fn new(
        position: Vector,
        tile_size: Vector,
        rotation: f32,
        texture: String,
        z: i8,
        travel: bool,
    ) -> Tile {
        let shape = Rectangle::new(position, tile_size);
        Tile {
            shape,
            texture,
            rotation,
            z,
            travel,
        }
    }

}

pub struct Map {
    tiles: Vec<Tile>,
    size: Vector,    // in number of tiles
    overlap: Vector, // amount of px that tiles overlap
    tile_size: Vector,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: Vec::new(),
            size: Vector::new(0, 0),
            overlap: Vector::new(0, 0),
            tile_size: Vector::new(0, 0),
        }
    }
    pub fn load(path: impl AsRef<Path> + 'static) -> Asset<String> {
        Asset::new(
            load_file(path).and_then(|contents| {
                Ok(String::from_utf8(contents).expect("The file must be UTF-8"))
            }),
        )
    }

    pub fn from_string(data: &mut String) -> Map {
        let mut lines = data.lines();

        let size = get_vec(lines.next().unwrap());
        let overlap = get_vec(lines.next().unwrap());
        let tile_size = get_vec(lines.next().unwrap());
        let mut tiles = Vec::with_capacity((size.x * size.y) as usize);

        for y in 0..size.y as usize {
            for x in 0..size.x as usize {
                let position = Vector::new(
                    x as f32 * (tile_size.x - overlap.x),
                    y as f32 * (tile_size.y - overlap.y),
                );
                let tile = build_tile(lines.next().unwrap(), position, tile_size);
                tiles.push(tile);
            }
        }

        Map {
            tiles,
            size,
            overlap,
            tile_size,
        }
    }

    pub fn draw_map(
        &mut self,
        view: &Rectangle,
        window: &mut Window,
        assets: &mut GameAssets,
    ) -> Result<()> {
        // figure out what tiles to start drawing from
        let x_start = (view.pos.x / (self.tile_size.x - self.overlap.x)).floor() as usize;
        let y_start = (view.pos.y / (self.tile_size.y - self.overlap.y)).floor() as usize;
        // figure out how many tiles to draw per row/colum
        let mut x_end =
            (view.size.x / (self.tile_size.x - self.overlap.x)).ceil() as usize + x_start + 1;
        let mut y_end =
            (view.size.y / (self.tile_size.y - self.overlap.y)).ceil() as usize + y_start + 1;

        if x_start + x_end > self.size.x as usize {
            x_end = self.size.x as usize;
        }

        if y_start + y_end > self.size.y as usize {
            y_end = self.size.y as usize;
        }

        assets.atlas.execute(|atlas| {
            for y in y_start..y_end {
                for x in x_start..x_end {
                    let tile = &self.tiles[(y as f32 * self.size.y + x as f32) as usize];
                    let texture = &tile.texture;
                    let img = &atlas.get(texture).unwrap().unwrap_image();
                    let rect = img.area().with_center(tile.shape.center());
                    window.draw_ex(
                        &rect,
                        Background::Img(&img),
                        Transform::rotate(tile.rotation),
                        tile.z,
                    );
                }
            }
            Ok(())
        })
    }
}
fn get_vec(line: &str) -> Vector {
    let mut xy = line.split(": ").last().unwrap().split(", ");
    let x = xy.next().unwrap().parse::<f32>().unwrap();
    let y = xy.next().unwrap().parse::<f32>().unwrap();
    Vector::new(x, y)
}

fn build_tile(line: &str, pos: Vector, tsize: Vector) -> Tile {
    let mut val = line.split(": ").last().unwrap().split(", ");
    let texture = val.next().unwrap().parse::<String>().unwrap();
    let angle = val.next().unwrap().parse::<f32>().unwrap();
    let z = val.next().unwrap().parse::<i8>().unwrap();
    let travel = val.next().unwrap().parse::<bool>().unwrap();
    Tile::new(pos, tsize, angle, texture, z, travel)
}