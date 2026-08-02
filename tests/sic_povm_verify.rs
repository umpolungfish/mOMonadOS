use std::process::Command;

#[test]
fn test_python_sic_povm_verification() {
    let status = Command::new("python3")
        .arg("tests/sic_verify.py")
        .status()
        .expect("failed to execute python3 tests/sic_verify.py");
    assert!(status.success(), "Python verification script failed");
}
