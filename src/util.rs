use crate::vector::Vector;

// http://jeffreythompson.org/collision-detection/poly-rect.php

pub fn line_line(
    line_a_1: &Vector,
    line_a_2: &Vector,
    line_b_1: &Vector,
    line_b_2: &Vector,
) -> bool {
    let ua: f32 = ((line_b_2.x - line_b_1.x) * (line_a_1.y - line_b_1.y)
        - (line_b_2.y - line_b_1.y) * (line_a_1.x - line_b_1.x))
        / ((line_b_2.y - line_b_1.y) * (line_a_2.x - line_a_1.x)
            - (line_b_2.x - line_b_1.x) * (line_a_2.y - line_a_1.y));
    let ub: f32 = ((line_a_2.x - line_a_1.x) * (line_a_1.y - line_b_1.y)
        - (line_a_2.y - line_a_1.y) * (line_a_1.x - line_b_1.x))
        / ((line_b_2.y - line_b_1.y) * (line_a_2.x - line_a_1.x)
            - (line_b_2.x - line_b_1.x) * (line_a_2.y - line_a_1.y));

    (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub)
}

pub fn line_rect(line_1: &Vector, line_2: &Vector, rect_pos: &Vector, rect_size: &Vector) -> bool {
    let left: bool = line_line(
        line_1,
        line_2,
        rect_pos,
        &Vector::new(rect_pos.x, rect_pos.y + rect_size.y),
    );
    let right: bool = line_line(
        line_1,
        line_2,
        &Vector::new(rect_pos.x + rect_size.x, rect_pos.y),
        &Vector::new(rect_pos.x + rect_size.x, rect_pos.y + rect_size.y),
    );
    let top: bool = line_line(
        line_1,
        line_2,
        rect_pos,
        &Vector::new(rect_pos.x + rect_size.x, rect_pos.y),
    );
    let bottom: bool = line_line(
        line_1,
        line_2,
        &Vector::new(rect_pos.x, rect_pos.y + rect_size.y),
        &Vector::new(rect_pos.x + rect_size.x, rect_pos.y + rect_size.y),
    );

    left || right || top || bottom
}

pub fn polygon_point(vertices: [[f64; 2]; 3], point: Vector) -> bool {
    let mut collision: bool = false;
    let mut next: usize;

    let mut current: usize = 0;
    while current < vertices.len() {
        next = current + 1;

        if next == vertices.len() {
            next = 0
        }

        let vc: [f64; 2] = vertices[current];
        let vn: [f64; 2] = vertices[next];

        if ((vc[1] as f32 > point.y && (vn[1] as f32) < point.y)
            || ((vc[1] as f32) < point.y && vn[1] as f32 > point.y))
            && (point.x
                < (vn[0] as f32 - vc[0] as f32) * (point.y - vc[1] as f32)
                    / (vn[1] as f32 - vc[1] as f32)
                    + vc[0] as f32)
        {
            collision = !collision
        }

        current += 1;
    }

    collision
}
