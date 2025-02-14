// https://www.r-5.org/files/books/computers/algo-list/realtime-3d/Christer_Ericson-Real-Time_Collision_Detection-EN.pdf

// The stuff here is going to be used for capsules whenever I make that

use vek::Vec3;

const EPSILON: f32 = 0.01;

/// Returns the squared distance between point c and segment ab
pub fn sq_dist_point_segment(a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>) -> f32 {
    let (ab, ac, bc) = (b - a, c - a, c - b);
    let e = ac.dot(ab);
    if e <= 0.0 {
        return ac.dot(ac);
    }
    let f = ab.dot(ab);
    if e >= f {
        return bc.dot(bc);
    }
    ac.dot(ac) - e.powi(2) / f
}

// I have no clue what any of this means but I'll put it here for whoever this is useful to
// Computes closest points C1 and C2 of S1(s)=P1+s*(Q1-P1) and
// S2(t)=P2+t*(Q2-P2), returning s and t. Function result is squared
// distance between S1(s) and S2(t)

/*

    I'm just gonna deal with this later, it's found on page 188 of Christer Ericson's book on Real-Time Collision Detection

pub fn closest_pt_segment_segment(
    p1: Vec3<f32>,
    q1: Vec3<f32>,
    p2: Vec3<f32>,
    q2: Vec3<f32>,
    // s: f32,
    // t: f32,
    // c1: Vec3<f32>,
    // c2: Vec3<f32>,
) -> (f32, f32, Vec3<f32>, Vec3<f32>, f32) {
    let (d1, d2) = (q1 - p1, q2 - p2); // Direction vectors of segments S1 and S2
    let r = p1 - p2;
    let (a, e) = (d1.dot(d1), d2.dot(d2)); // Squared length of segments S1 and S2 respectively, always nonnegative
    let f = d2.dot(r);

    if a <= EPSILON && e <= EPSILON {
        return (0.0, 0.0, p1, p2, (p1 - p2).dot(p1 - p2));
    }

}
*/
