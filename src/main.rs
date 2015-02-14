#![feature(core)]
#![feature(std_misc)]

use std::num::Float;


fn main() {
    let p = FlatPoint{x: 0.0, y: 0.0};
    let q = FlatPoint{x: 1.0, y: 1.0};

    println!("{:?}", p);
    println!("distance={}", distance(&p, &q));
    
    let sp = SpacePoint{a: 0.0, b: 0.0, c: 0.0,};
    let sq = SpacePoint{a: 1.0, b: 1.0, c: 0.0,};
    println!("distance={}", distance(&sp, &sq));
}

//enum CoordinateSystem {
//  Cartesian,
//  Polar
//}

trait Point {
    type F: Float;
    fn get(&self, d: u32) -> Self::F;
    fn dimension() -> u32;
}

trait Curve {
  type P: Point;
  //fn length(&self) -> Self::P::F;
  fn length(&self) -> f64;
  fn start_point(&self) -> Self::P;
  fn end_point(&self) -> Self::P;
  fn is_closed(&self) -> bool;
}

trait LineString: Curve {
  fn num_points() -> u32;
  fn point(u32) -> Self::P;
}

fn distance<P: Point>(a: &P, b: &P) -> P::F {
    //todo: P::dimension() will work in the future.

    range(0, (<P as Point>::dimension())).map(|i| a.get(i) - b.get(i))
      .fold(Float::zero(), | sum, dist| sum + dist * dist).sqrt()
}

#[derive(Debug)]
struct FlatPoint {
    x: f32,
    y: f32,
}

impl Point for FlatPoint {
    type F = f32;
    fn get(&self, d: u32) -> f32 {
        match d {
            0 => self.x,
            1 => self.y,
            _ => panic!()
        }
    }
    fn dimension() -> u32 {
        2
    }
}

#[derive(Debug)]
struct FlatLine {
  points: Vec<FlatPoint>,
}

impl Curve for FlatLine {
  type P = FlatPoint;
  fn length() -> f64 {
    3.3
  }
  fn start_point() -> FlatPoint {
    self.points.begin()
  }
}

#[derive(Debug)]
struct SpacePoint {
    a: f64,
    b: f64,
    c: f64
}

impl Point for SpacePoint {
    type F = f64;
    fn get(&self, d: u32) -> f64 {
        match d {
            0 => self.a,
            1 => self.b,
            2 => self.c,
            _ => panic!()
        }
    }
    fn dimension() -> u32 {
        3
    }
}

