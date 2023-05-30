use test::Bencher;

#[bench]
fn nested_for_100(b: &mut Bencher) {
	let songs = crate::songs(100);
	b.iter(|| crate::nested_for(&songs))
} 

#[bench]
fn pivot_100(b: &mut Bencher) {
	let songs = crate::songs(100);
	b.iter(|| crate::pivot(&songs))
} 

#[bench]
fn nested_for_1000(b: &mut Bencher) {
	let songs = crate::songs(1000);
	b.iter(|| crate::nested_for(&songs))
} 

#[bench]
fn pivot_1000(b: &mut Bencher) {
	let songs = crate::songs(1000);
	b.iter(|| crate::pivot(&songs))
} 

#[bench]
fn nested_for_10000(b: &mut Bencher) {
	let songs = crate::songs(10000);
	b.iter(|| crate::nested_for(&songs))
} 

#[bench]
fn pivot_10000(b: &mut Bencher) {
	let songs = crate::songs(10000);
	b.iter(|| crate::pivot(&songs))
} 
