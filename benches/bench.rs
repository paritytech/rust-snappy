#![feature(test)]
extern crate test;

extern crate parity_snappy as snappy;
extern crate rand;

#[cfg(test)]
mod tests {
	use rand::prelude::*;
	use snappy;
	use test::Bencher;

	const INPUT_SIZE: usize = 1 << 19;

	#[bench]
	fn bench_compress_decompress(b: &mut Bencher) {
		let mut rng = StdRng::from_seed([0u8; 32]);
		let mut input = [0u8; INPUT_SIZE];
		rng.fill(&mut input[..]);

		let mut compressed = Vec::with_capacity(INPUT_SIZE);
		let mut decompressed = Vec::with_capacity(INPUT_SIZE);

		b.iter(|| {
			let size = snappy::compress_into(&input, &mut compressed);
			let _ = snappy::decompress_into(&compressed[..size], &mut decompressed);
		});
	}
}
