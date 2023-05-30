#[test]
fn nested_for() {
	assert_eq!(4, crate::nested_for(&[50, 10, 50, 10]))
}

#[test]
fn pivot_idempotency() {
	for i in 0..60 {
		assert_eq!(0, crate::pivot(&[i]));
	}
}

#[test]
fn pivot_base_case_0() {
	assert_eq!(1, crate::pivot(&[60, 60]));
	assert_eq!(3, crate::pivot(&[60, 60, 60]));
}

#[test]
fn pivot_base_case_30() {
	assert_eq!(1, crate::pivot(&[30, 30]));
	assert_eq!(3, crate::pivot(&[30, 30, 30]));
}

#[test]
fn pivot_blackbox() {
	let songs = crate::songs(60);

	assert_eq!(crate::nested_for(&songs), crate::pivot(&songs));
}
