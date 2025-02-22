//! This test module tests against simple files generated by the pygmsh package.

use nom::IResult;
use vtkio::model::*;
use vtkio::parser::*;
use vtkio::writer::*;
use vtkio::Error;

macro_rules! test_b {
    ($fn:ident ($in:expr, $($args:expr),*) => $out:expr) => {
        assert_eq!($fn($in, $($args),*), IResult::Ok(("".as_bytes(), $out.clone())));
    };
    ($fn:ident ($in:expr) => $out:expr) => {
        assert_eq!($fn($in), IResult::Ok(("".as_bytes(), $out.clone())));
    };
}

macro_rules! test_ignore_rem {
    ($fn:ident ($in:expr, $($args:expr),*) => $out:expr) => {
        {
            let result = $fn($in, $($args),*);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().1, $out.clone());
        }
    };
    ($fn:ident ($in:expr) => $out:expr) => {
        {
            let result = $fn($in);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().1, $out.clone());
        }
    };
}

type Result = std::result::Result<(), Error>;

// Helper functions to convert between endianness.

fn ne(vtk: &Vtk) -> Vtk {
    Vtk {
        byte_order: ByteOrder::native(),
        ..vtk.clone()
    }
}

fn le(vtk: &Vtk) -> Vtk {
    Vtk {
        byte_order: ByteOrder::LittleEndian,
        ..vtk.clone()
    }
}

fn make_test_file(leading_zero_offset: bool) -> Vtk {
    Vtk {
        version: Version::new((5, 1)),
        byte_order: ByteOrder::BigEndian,
        title: String::from("written by meshio v5.3.0"),
        file_path: None,
        data: DataSet::inline(UnstructuredGridPiece {
            points: vec![
                0.0f64,
                0.0,
                0.0,
                1.0,
                -0.2,
                0.0,
                1.1,
                1.2,
                0.0,
                0.1,
                0.7,
                0.0,
                0.3333333333325021,
                -0.06666666666650042,
                0.0,
                0.6666666666657866,
                -0.1333333333331573,
                0.0,
                1.0249999999999424,
                0.14999999999919245,
                0.0,
                1.0499999999998704,
                0.4999999999981836,
                0.0,
                1.074999999999934,
                0.8499999999990746,
                0.0,
                0.766666666667985,
                1.0333333333339925,
                0.0,
                0.433333333334733,
                0.8666666666673664,
                0.0,
                0.050000000000122564,
                0.3500000000008579,
                0.0,
                0.7444729167676052,
                0.3524793413776178,
                0.0,
                0.3781088913238718,
                0.4816987298113132,
                0.0,
                0.7412636346823331,
                0.6806963451979247,
                0.0,
                0.5070791452210437,
                0.16277273408010906,
                0.0,
                0.253704273975508,
                0.18556095944515594,
                0.0,
                0.7797139636550688,
                0.08823831456107314,
                0.0,
            ]
            .into(),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    offsets: {
                        let mut offsets = vec![
                            2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 27, 30, 33, 36, 39, 42, 45,
                            48, 51, 54, 57, 60, 63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 91, 92, 93,
                            94,
                        ];
                        if leading_zero_offset {
                            offsets.insert(0, 0);
                        }
                        offsets
                    },
                    connectivity: vec![
                        0, 4, 4, 5, 5, 1, 1, 6, 6, 7, 7, 8, 8, 2, 2, 9, 9, 10, 10, 3, 3, 11, 11, 0,
                        10, 13, 14, 13, 12, 14, 10, 3, 13, 8, 2, 9, 4, 5, 15, 9, 10, 14, 3, 11, 13,
                        6, 7, 12, 8, 9, 14, 7, 8, 14, 12, 7, 14, 15, 5, 17, 5, 1, 17, 1, 6, 17, 12,
                        13, 15, 13, 11, 16, 15, 13, 16, 11, 0, 16, 0, 4, 16, 6, 12, 17, 12, 15, 17,
                        4, 15, 16, 0, 1, 2, 3,
                    ],
                },
                types: vec![
                    vec![CellType::Line; 12],
                    vec![CellType::Triangle; 22],
                    vec![CellType::Vertex; 4],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<CellType>>(),
            },
            data: Attributes {
                point: vec![],
                cell: vec![],
            },
        }),
    }
}

#[test]
fn legacy_ascii() -> Result {
    let input = include_str!("../assets/pygmsh/ascii.vtk").as_bytes();
    let out1 = make_test_file(true);
    assert!(parse_be(input).is_ok());
    test_ignore_rem!(parse_be(input) => out1);
    let mut outtest = String::new();
    outtest.write_vtk_ne(out1.clone())?;
    // println!("{}", outtest);
    test_b!(parse_ne(String::new().write_vtk_ne(out1.clone())?.as_bytes()) => ne(&out1));
    test_b!(parse_ne(Vec::<u8>::new().write_vtk_ne(out1.clone())?) => ne(&out1));
    test_b!(parse_le(Vec::<u8>::new().write_vtk_le(out1.clone())?) => le(&out1));
    test_b!(parse_be(Vec::<u8>::new().write_vtk_be(out1.clone())?) => out1);
    Ok(())
}

#[test]
fn legacy_binary() -> Result {
    let input = include_bytes!("../assets/pygmsh/binary.vtk");
    let out1 = make_test_file(true);
    assert!(parse_be(input).is_ok());
    test_ignore_rem!(parse_be(input) => out1);
    let mut outtest = String::new();
    outtest.write_vtk_ne(out1.clone())?;
    // println!("{}", outtest);
    test_b!(parse_ne(String::new().write_vtk_ne(out1.clone())?.as_bytes()) => ne(&out1));
    test_b!(parse_ne(Vec::<u8>::new().write_vtk_ne(out1.clone())?) => ne(&out1));
    test_b!(parse_le(Vec::<u8>::new().write_vtk_le(out1.clone())?) => le(&out1));
    test_b!(parse_be(Vec::<u8>::new().write_vtk_be(out1.clone())?) => out1);
    Ok(())
}

/// Ensures that points from the two given vtk files are equivalent up to floating point error, and then overwrites
/// the first input to match exactly to the points in the second input, so the can be compared using `PartialEq` later.
#[cfg(feature = "xml")]
fn compare_points_in_float_and_overwrite(vtu: &mut Vtk, expected: &Vtk) {
    let expected_points = if let DataSet::UnstructuredGrid { ref pieces, .. } = expected.data {
        pieces[0]
            .load_piece_data(None)
            .unwrap()
            .points
            .cast_into::<f64>()
            .unwrap()
    } else {
        panic!("Wring vtk data type");
    };

    // Compare positions via floating point comparisons.
    if let DataSet::UnstructuredGrid { pieces, .. } = &mut vtu.data {
        let piece = &mut pieces[0];
        if let Piece::Inline(piece_data) = piece {
            let mut points = piece_data
                .points
                .cast_into::<f64>()
                .expect("Point have the wrong type.");
            for (i, (point, &expected_point)) in
                points.iter_mut().zip(expected_points.iter()).enumerate()
            {
                if (*point - expected_point).abs() > 1e-6 * expected_point.abs() {
                    eprintln!("{}: actual {} vs. expected {}", i, *point, expected_point);
                }
                assert!((*point - expected_point).abs() <= 1e-6 * expected_point.abs());
                *point = expected_point; // match test data for full comparison later.
            }
            piece_data.points = points.into();
        } else {
            panic!("Loaded vtk file has no inline unstructured grid piece");
        }
    } else {
        panic!("Loaded vtk file is not an unstructured grid");
    }
}

/// Ensures the given xml based vtk file has the right values and overwrites them to match
/// the asset returned by make_test_file.
#[cfg(feature = "xml")]
fn assert_and_fix_xml_vtu(vtu: &mut Vtk) {
    vtu.file_path = None; // Reset file path to satisfy comparison
    assert_eq!(vtu.version, Version::new((0, 1))); // XML file version is ignored.
    vtu.version = (5, 1).into(); // Explicitly set version to satisfy comparison.
    assert_eq!(vtu.title, String::new()); // Default empty title
    vtu.title = "written by meshio v5.3.0".into(); // Match test file
    vtu.byte_order = ByteOrder::BigEndian; // Match test file
}

#[test]
#[cfg(feature = "xml")]
fn xml_ascii() -> Result {
    let mut vtu = Vtk::import("./assets/pygmsh/ascii.vtu")?;
    assert_and_fix_xml_vtu(&mut vtu);
    let expected = make_test_file(false);
    compare_points_in_float_and_overwrite(&mut vtu, &expected);
    assert_eq!(vtu, expected);
    Ok(())
}

#[test]
#[ignore]
#[cfg(feature = "xml")]
#[cfg(feature = "xz2")]
fn xml_lzma() -> Result {
    let mut vtu = Vtk::import("./assets/pygmsh/lzma.vtu")?;
    assert_and_fix_xml_vtu(&mut vtu);
    let expected = make_test_file(false);
    compare_points_in_float_and_overwrite(&mut vtu, &expected);
    assert_eq!(vtu, expected);
    Ok(())
}

#[test]
#[cfg(feature = "xml")]
fn xml_no_compression() -> Result {
    let mut vtu = Vtk::import("./assets/pygmsh/no-compression.vtu")?;
    assert_and_fix_xml_vtu(&mut vtu);
    let expected = make_test_file(false);
    compare_points_in_float_and_overwrite(&mut vtu, &expected);
    assert_eq!(vtu, expected);
    Ok(())
}

#[test]
#[ignore]
#[cfg(feature = "xml")]
#[cfg(feature = "flate2")]
fn xml_zlib() -> Result {
    let mut vtu = Vtk::import("./assets/pygmsh/zlib.vtu")?;
    assert_and_fix_xml_vtu(&mut vtu);
    let expected = make_test_file(false);
    compare_points_in_float_and_overwrite(&mut vtu, &expected);
    assert_eq!(vtu, expected);
    Ok(())
}
