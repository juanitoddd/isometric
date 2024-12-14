use macroquad::prelude::*;

pub struct Point {  
  pub x: f32,
  pub y: f32,  
}

pub struct Tile {  
  pub lt: Point,
  pub rt: Point,
  pub lb: Point,
  pub rb: Point,
  pub bg:Color
}

impl Tile {
  pub fn collides_with(&self, other: &Self) -> bool {
      self.rect().overlaps(&other.rect())
  }

  fn rect(&self) -> Rect {
      Rect {
          x: self.x - self.size / 2.0,
          y: self.y - self.size / 2.0,
          w: self.size,
          h: self.size,
      }
  }

  pub fn render(&self) {
    draw_line(lt.x, lt.y, rt.x, rt.y, 1.0, BLACK);
    draw_line(rt.x, rt.y, rb.x, rb.y, 1.0, BLACK);
    draw_line(lt.x, lt.y, lb.x, lb.y, 1.0, BLACK);
    draw_line(lb.x, lb.y, rb.x, rb.y, 1.0, BLACK);
  }
}