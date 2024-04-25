extern crate fitrs;

use std::process::Command;

use fitrs::{Fits, Hdu};

#[test]
fn write_single_hdu_file() {
    let data = (0..20).flat_map(|i| (0..20).map(move |j| i + j)).collect();
    let primary_hdu = Hdu::new(&[20, 20], data);
    let _ = Fits::create("out.fits", primary_hdu).expect("created!");

    let output = Command::new("verify/verify.py")
        .arg("out.fits")
        .output()
        .expect("Failed to execute verify command");

    assert_eq!(output.stdout, vec![], "No standard output");
    assert_eq!(output.stderr, vec![], "No error output");
    assert!(output.status.success(), "Exit with success");
}

#[test]
fn write_multiple_hdu_file() {
    let data: Vec<i32> = (0..20).flat_map(|i| (0..20).map(move |j| i + j)).collect();
    let primary_hdu = Hdu::new(&[20, 20], data.clone());
    let mut fits = Fits::create("out.fits", primary_hdu).expect("created!");

    let secondary_hdu = Hdu::new(&[20, 20], data);
    fits.push(secondary_hdu).expect("Pushed");

    let output = Command::new("verify/verify.py")
        .arg("out.fits")
        .output()
        .expect("Failed to execute verify command");

    assert_eq!(output.stdout, vec![], "No standard output");
    assert_eq!(output.stderr, vec![], "No error output");
    assert!(output.status.success(), "Exit with success");
}

#[test]
fn write_file_with_long_string() {
    let mut primary_hdu = Hdu::new(&[1], vec![0]);
    primary_hdu.insert(
        "TEST",
        "This is a very long string value that is continued over more than one keyword.",
    );
    let _ = Fits::create("out.fits", primary_hdu).expect("created!");

    let output = Command::new("verify/verify.py")
        .arg("out.fits")
        .output()
        .expect("Failed to execute verify command");

    assert_eq!(output.stdout, vec![], "No standard output");
    assert_eq!(output.stderr, vec![], "No error output");
    assert!(output.status.success(), "Exit with success");
}

#[test]
fn write_file_with_empty_hdu() {
    let primary_hdu = Hdu::empty();
    let _ = Fits::create("out.fits", primary_hdu).expect("created!");

    let output = Command::new("verify/verify.py")
        .arg("out.fits")
        .output()
        .expect("Failed to execute verify command");

    assert_eq!(output.stdout, vec![], "No standard output");
    assert_eq!(output.stderr, vec![], "No error output");
    assert!(output.status.success(), "Exit with success");
}
