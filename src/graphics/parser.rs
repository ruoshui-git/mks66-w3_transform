// :( Oh my God! This script spec is designed in a way that a parser library is generally useless!!!
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::process::Command;

use super::{matrix::Matrix, PPMImg};

#[allow(dead_code)]
pub struct DWScript {
    filename: String,
    edges: Matrix,
    trans: Matrix,
    img: PPMImg,
    tmpfile_name: String,
}

#[allow(dead_code)]
/// Advances a line iterator and panic on error
fn getline_or_error(
    line: &mut impl Iterator<Item = (usize, io::Result<String>)>,
) -> (usize, String) {
    if let Some((num, line)) = line.next() {
        let line = line.expect("Error while reading line").trim().to_string();
        (num, line)
    } else {
        panic!("Error reading line");
    }
}

/// Parse floats from a line and return them in a vec. Panic on error.
fn parse_floats(line: String) -> Vec<f64> {
    line.split(' ')
        .map(|x| x.parse::<f64>().expect("Error parsing numbers"))
        .collect()
}

#[allow(dead_code)]
impl DWScript {
    pub fn new(filename: &str) -> Self {
        DWScript {
            filename: filename.to_string(),
            edges: Matrix::new(0, 4, vec![]),
            trans: Matrix::ident(4),
            img: PPMImg::new(500, 500, 255),
            tmpfile_name: String::from("tmp.ppm"),
        }
    }

    pub fn do_parse(&mut self) -> Matrix {
        let _f = File::open(&self.filename).expect("Error opening file");
        let f = BufReader::new(_f);
        let mut lines = f.lines().enumerate();
        while let Some((num, line)) = lines.next() {
            let line = line.expect("Error while reading file");

            match line.trim() {
                "line" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let mut pts: Vec<f64> = parse_floats(dline);
                    assert_eq!(6, pts.len());
                    self.edges.append_edge(&mut pts);
                }
                "ident" => {
                    self.trans = Matrix::ident(4);
                }
                "scale" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let scale: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, scale.len());
                    self.trans = self.trans.mul(&Matrix::scale(scale[0], scale[1], scale[2]));
                }
                "move" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let mv: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, mv.len());
                    self.trans = self.trans.mul(&Matrix::mv(mv[0], mv[1], mv[2]));
                }
                "rotate" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v: Vec<&str> = dline.split(' ').collect();
                    let (scale, deg): (&str, f64) =
                        (v[0], v[1].parse().expect("Error parsing number"));
                    let rotate = match scale {
                        "x" => Matrix::rotatex(deg),
                        "y" => Matrix::rotatey(-deg),
                        "z" => Matrix::rotatez(deg),
                        _ => panic!("Unknown rotation axis on line {}", _dnum),
                    };
                    self.trans = self.trans.mul(&rotate);
                }
                "apply" => {
                    self.edges = self.edges.mul(&self.trans);
                }
                "display" => {
                    self.img.clear();
                    self.img.render_edge_matrix(&self.edges);
                    self.img
                        .write_binary(self.tmpfile_name.as_str())
                        .expect("Error writing to file");

                    let mut display = Command::new("display")
                        .arg(self.tmpfile_name.as_str())
                        .spawn()
                        .unwrap();
                    let _result = display.wait().unwrap();
                    fs::remove_file(self.tmpfile_name.as_str()).expect("Error removing tmp file");
                }
                "save" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    self.img.clear();
                    self.img.render_edge_matrix(&self.edges);
                    self.img
                        .write_binary(dline.as_str())
                        .expect("Error writing to file");
                    if dline.ends_with(".png") {
                        Command::new("display")
                            .arg(dline.as_str())
                            .arg(dline.as_str())
                            .spawn()
                            .unwrap();
                    }
                }
                x if x.starts_with("\\") => {}
                _ => panic!("Unrecognized command on line {}: {}", num, line),
            }
        }

        self.edges.clone()
    }
}
