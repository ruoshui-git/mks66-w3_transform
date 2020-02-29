mod graphics;

use graphics::PPMImg;
use graphics::matrix::Matrix;


fn main() {
    let mut img = PPMImg::new(500, 500, 255);


    
    let cube = vec![
        // front points
        0.0, 0.0, 0.0, 1.0,
        100.0, 0.0, 0.0, 1.0,

        100.0, 0.0, 0.0, 1.0,
        100.0, 100.0, 0.0, 1.0,

        100.0, 100.0, 0.0, 1.0,
        0.0, 100.0, 0.0, 1.0,

        0.0, 100.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 1.0,

        0.0, 0.0, 100.0, 1.0,
        100.0, 0.0, 100.0, 1.0,


        // square on z = 100
        100.0, 0.0, 100.0, 1.0,
        100.0, 100.0, 100.0, 1.0,
        0.0, 0.0, 100.0, 1.0,
        0.0, 100.0, 100.0, 1.0,
        100.0, 100.0, 100.0, 1.0,
        0.0, 100.0, 100.0, 1.0,

        // z=0 -> z=100
        0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 100.0, 1.0,

        0.0, 100.0, 0.0, 1.0,
        0.0, 100.0, 100.0, 1.0,

        100.0, 100.0, 0.0, 1.0,
        100.0, 100.0, 100.0, 1.0,

        100.0, 0.0, 0.0, 1.0,
        100.0, 0.0, 100.0, 1.0,
    ];

    let m = Matrix::new_from_vec(cube.len() / 4, 4, &cube);

    let m = m
    .mul(&Matrix::scale(2.0, 2.0, 2.0))
    .mul(&Matrix::mv(100.0, 100.0, 0.0))
    .mul(&Matrix::rotatez(20.0))
    .mul(&Matrix::rotatex(20.0))
    .mul(&Matrix::rotatey(20.0))
    .mul(&Matrix::rotatey(20.0))
    ;
    
    // let t = Matrix::ident(4)
    //     .mul(&Matrix::scale(2, 2, 2))

    // let m = m.mul(&t);

    img.render_edge_matrix(&m);
    img.write_ascii("img.ppm").expect("Error writing to file");
}
