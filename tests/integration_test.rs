
use w4_trans::graphics::{PPMImg, matrix::Matrix};
use std::process::Command;

#[test]
fn test_cube() {
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



        // square on z = 100
        0.0, 0.0, 100.0, 1.0,
        100.0, 0.0, 100.0, 1.0,
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

    let m = Matrix::new_clone_vec(cube.len() / 4, 4, &cube);

    // let t = Matrix::ident(4)
    // .transposed_mul(&Matrix::mv(100.0, 100.0, 0.0))
    // .transposed_mul(&Matrix::scale(2.0, 2.0, 2.0))
    // .transposed_mul(&Matrix::rotatez(20.0))
    // .transposed_mul(&Matrix::rotatex(20.0))
    // .transposed_mul(&Matrix::rotatey(20.0))
    // .transposed_mul(&Matrix::rotatey(20.0))
    // ;
    // let m = t.transposed_mul(&m);
    
    let t = Matrix::ident(4)
    .mul(&Matrix::mv(-50.0, -50.0, 0.0))
    .mul(&Matrix::scale(2.0, 2.0, 2.0))
    .mul(&Matrix::rotatez(20.0))
    .mul(&Matrix::rotatex(20.0))
    .mul(&Matrix::rotatey(-20.0))
    .mul(&Matrix::rotatey(-20.0))
    .mul(&Matrix::mv(250.0, 250.0, 0.0))
        ;
    let m = m.mul(&t);

    img.render_edge_matrix(&m);
    img.render_edge_matrix(&Matrix::new_clone_vec(24, 4, &cube));

    let filename = "img.ppm";
    let cmd = "display";
    img.write_binary(filename).expect("Error writing to file");

    let mut display = Command::new(cmd).arg(filename).spawn().unwrap();
    let _result = display.wait().unwrap();
}