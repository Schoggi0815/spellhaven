use crate::{gradient, permutationtable::NoiseHasher, vectors::Vector2};

// Skew Value
//
//     sqrt(n + 1) - 1
// F = ---------------
//            n
fn skew_factor(n: usize) -> f64 {
    let n = n as f64;

    ((n + 1.).sqrt() - 1.) / n
}

//  Unskew Value
//
//     1 - 1 / sqrt(n + 1)
// G = -------------------
//             n
fn unskew_factor(n: usize) -> f64 {
    let n = n as f64;

    (1. - (1. / (n + 1.).sqrt())) / n
}

/// The simplex noise code was adapted from code by Stefan Gustavson,
/// http://staffwww.itn.liu.se/~stegu/aqsis/aqsis-newnoise/sdnoise1234.c
///
/// This is Stefan Gustavson's original copyright notice:
///
/// /* sdnoise1234, Simplex noise with true analytic
///  * derivative in 1D to 4D.
///  *
///  * Copyright © 2003-2011, Stefan Gustavson
///  *
///  * Contact: stefan.gustavson@gmail.com
///  *
///  * This library is public domain software, released by the author
///  * into the public domain in February 2011. You may do anything
///  * you like with it. You may even remove all attributions,
///  * but of course I'd appreciate it if you kept my name somewhere.
///  *
///  * This library is distributed in the hope that it will be useful,
///  * but WITHOUT ANY WARRANTY; without even the implied warranty of
///  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
///  * General Public License for more details.
///
///  */
#[inline(always)]
pub fn simplex_2d<NH>(point: Vector2<f64>, hasher: &NH) -> (f64, [f64; 2])
where
    NH: NoiseHasher + ?Sized,
{
    let skew_factor: f64 = skew_factor(2);
    let unskew_factor: f64 = unskew_factor(2);

    // Skew the input space to determine which simplex cell we're in
    let skew = point.sum() * skew_factor;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast().unwrap();

    let unskew: f64 = floor.sum() * unskew_factor;
    // Unskew the cell origin back to (x,y) space
    let unskewed = floor - unskew;
    // The x,y distances from the cell origin
    let offset1 = point - unskewed;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    let order = if offset1.x > offset1.y {
        // Offsets for second (middle) corner of simplex in (i,j) coords
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        Vector2::new(1.0, 0.0)
    } else {
        // upper triangle, YX order: (0,0)->(0,1)->(1,1)
        Vector2::new(0.0, 1.0)
    };

    // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
    // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
    // c = (3-sqrt(3))/6

    // Offsets for middle corner in (x,y) unskewed coords
    let offset2 = offset1 - order + unskew_factor;
    // Offsets for last corner in (x,y) unskewed coords
    let offset3 = offset1 - 1.0 + 2.0 * unskew_factor;

    // Calculate gradient indexes for each corner
    let gi0 = hasher.hash(&cell.into_array());
    let gi1 = hasher.hash(&(cell + order.numcast().unwrap()).into_array());
    let gi2 = hasher.hash(&(cell + 1).into_array());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradient: Vector2<f64>,
    }

    #[inline(always)]
    fn surflet(
        gradient_index: usize,
        point: Vector2<f64>,
    ) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let gradient: Vector2<f64> = gradient::grad2(gradient_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(gradient),
                t,
                t2,
                t4,
                gradient,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradient: Vector2::zero(),
            }
        }
    }

    // Calculate the contribution from the three corners
    let corner0 = surflet(gi0, offset1);
    let corner1 = surflet(gi1, offset2);
    let corner2 = surflet(gi2, offset3);

    // Add contributions from each corner to get the final noise value.
    // The result is scaled to return values in the interval [-1, 1].
    let noise = corner0.value + corner1.value + corner2.value;

    // A straight, unoptimised calculation would be like:
    //   dnoise_dx = -8.0 * t20 * t0 * x0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gx0;
    //   dnoise_dy = -8.0 * t20 * t0 * y0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gy0;
    //   dnoise_dx += -8.0 * t21 * t1 * x1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gx1;
    //   dnoise_dy += -8.0 * t21 * t1 * y1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gy1;
    //   dnoise_dx += -8.0 * t22 * t2 * x2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gx2;
    //   dnoise_dy += -8.0 * t22 * t2 * y2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gy2;
    //
    let mut dnoise =
        offset1 * corner0.t2 * corner0.t * corner0.gradient.dot(offset1);
    dnoise += offset2 * corner1.t2 * corner1.t * corner1.gradient.dot(offset2);
    dnoise += offset3 * corner2.t2 * corner2.t * corner2.gradient.dot(offset3);

    dnoise *= -8.0;

    dnoise += corner0.gradient * corner0.t4
        + corner1.gradient * corner1.t4
        + corner2.gradient * corner2.t4;

    // dnoise *= 2.;

    (noise, dnoise.into())
}
