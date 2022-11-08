use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};

extern crate nalgebra as na;
use na::{Isometry2, Vector2};
use ncollide2d::shape::{Ball};

use std::f32::consts::PI;

use crate::components::{Hitbox, HitboxShape, Powerable};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CollisionType {
    _Through,
    Bounce {bounces: Option<i8>, sticks: bool},
}

pub struct Movable {
    pub dx: f32,
    pub dy: f32,
    pub power: Powerable,
    pub max_accel_force: f32,
    pub collision_type: CollisionType,
    pub prevent_collision_id: Option<u32>,
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

impl Movable {
    pub fn new(power:u8, max_accel_force: f32, collision_type: CollisionType) -> Movable {
        Movable {
            dx: 0.0,
            dy: 0.0,
            power: Powerable::new(power, power),
            max_accel_force,
            collision_type,
            prevent_collision_id: None,
        }
    }
}

pub fn get_movable_shape_pos(
    transform: &Transform,
    hitbox: &Hitbox,
) -> (Isometry2<f32>, Ball<f32>) {
    let x = transform.translation().x;
    let y = transform.translation().y;

    let rotation = transform.rotation();
    let (_, _, angle) = rotation.euler_angles();

    let collider_pos =
        Isometry2::new(Vector2::new(x, y), angle);
    let collider_shape =
        Ball::new(hitbox.width / 2.0);

    (collider_pos, collider_shape)
}

pub fn clean_angle(angle: f32) -> f32 {
    let mut new_angle: f32;

    if angle > PI {
        let diff = angle - PI;
        new_angle = -PI + diff;
    } else if angle < -PI {
        let diff = -PI - angle;
        new_angle = PI - diff;
    } else {
        new_angle = angle;
    }

    if new_angle > PI || new_angle < -PI {
        new_angle = clean_angle(new_angle);
    }

    new_angle
}

pub fn calc_bounce_angle(
    offset_x: f32,
    offset_y: f32,
    hitbox_hw: f32,
    hitbox_hh: f32,
    hitbox_shape: HitboxShape,
    moving_dx: f32,
    moving_dy: f32,
) -> (f32, f32) {
    match hitbox_shape {
        HitboxShape::Circle => {
            let moving_angle = moving_dy.atan2(moving_dx);
            let contact_angle = offset_y.atan2(offset_x);

            let contact_perp_angle = clean_angle(contact_angle + PI / 2.0);
            let new_angle = clean_angle(contact_perp_angle + (contact_perp_angle - moving_angle));
            
            let moving_speed = (moving_dx.powi(2) + moving_dy.powi(2)).sqrt();
            (moving_speed * new_angle.cos(), moving_speed * new_angle.sin())
        },
        HitboxShape::Rectangle => {
            match (offset_x, offset_y, hitbox_hw, hitbox_hh) {
                (x, y, w, h) if x > -w && x < w && y < h && y > -h => (-moving_dx, -moving_dy), //hit corner
                (x, _, w, _) if x > -w && x < w => (moving_dx, -moving_dy), //hit top or bottom wall
                (_, y, _, h) if y < h && y > -h => (-moving_dx, moving_dy), //hit left or right wall
                (_, _, _, _) => (-moving_dx, -moving_dy)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_clean_angle() {
        assert_approx_eq!(clean_angle(270. / 180. * PI), -90. / 180. * PI);
        assert_approx_eq!(clean_angle(-190. / 180. * PI), 170. / 180. * PI);
        assert_approx_eq!(clean_angle(90. / 180. * PI), 90. / 180. * PI);
        assert_approx_eq!(clean_angle(1800.0 / 180. * PI), 0. / 180. * PI, 0.0001);
        assert_approx_eq!(clean_angle(-1850.0 / 180. * PI), -50. / 180. * PI, 0.0001);
    }


    // Rectangle Bounce Testing

    #[test]
    fn test_calc_bounce_angle_rect_left_side_moving_up_right() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            1.0,
            0.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            1.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, -1.0);
        assert_approx_eq!(moving_dy_new, 1.0);
    }

    #[test]
    fn test_calc_bounce_angle_rect_top_side_moving_down_right() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            1.0,
            -1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 1.0);
        assert_approx_eq!(moving_dy_new, 1.0);
    }

    #[test]
    fn test_calc_bounce_angle_rect_top_side_moving_mostly_down_slightly_right() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            0.2,
            -2.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 0.2);
        assert_approx_eq!(moving_dy_new, 2.0);
    }

    #[test]
    fn test_calc_bounce_angle_rect_top_side_moving_slightly_down_mostly_right() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            3.5,
            -0.3, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 3.5);
        assert_approx_eq!(moving_dy_new, 0.3);
    }

    #[test]
    fn test_calc_bounce_angle_rect_top_side_moving_down_left() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            -1.0,
            -1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, -1.0);
        assert_approx_eq!(moving_dy_new, 1.0);
    }


    #[test]
    fn test_calc_bounce_angle_rect_bottom_side_moving_up_right() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            1.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 1.0);
        assert_approx_eq!(moving_dy_new, -1.0);
    }

    #[test]
    fn test_calc_bounce_angle_rect_bottom_side_moving_up_left() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //half width/height
            HitboxShape::Rectangle,
            -1.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, -1.0);
        assert_approx_eq!(moving_dy_new, -1.0);
    }

    // Circle Bounce Testing

    #[test]
    fn test_calc_bounce_angle_circle1() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            1.0,
            0.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //radius (w=h)
            HitboxShape::Circle,
            -1.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 1.0);
        assert_approx_eq!(moving_dy_new, 1.0);
    }

    #[test]
    fn test_calc_bounce_angle_circle2() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //radius (w=h)
            HitboxShape::Circle,
            1.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 1.0);
        assert_approx_eq!(moving_dy_new, -1.0);
    }

    #[test]
    fn test_calc_bounce_angle_circle3() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            0.0,
            -1.0, //hitbox contact offset (x,y)
            1.0,
            1.0, //radius (w=h)
            HitboxShape::Circle,
            0.125,
            0.9, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 0.125);
        assert_approx_eq!(moving_dy_new, -0.900);
    }

    #[test]
    fn test_calc_bounce_angle_circle4() {
        let (moving_dx_new, moving_dy_new) = calc_bounce_angle(
            1.0 / (2.0 as f32).sqrt(),
            -1.0 / (2.0 as f32).sqrt(), //hitbox contact offset (x,y)
            1.0,
            1.0, //radius (w=h)
            HitboxShape::Circle,
            0.0,
            1.0, //moving dx, dy
        );

        assert_approx_eq!(moving_dx_new, 1.0);
        assert_approx_eq!(moving_dy_new, 0.0);
    }
}
