// This file is auto-generated by gen_target.sh based on target_template.txt
// To modify it, modify target_template.txt and run gen_target.sh instead.

//Uncomment this for libfuzzer builds:
//#![no_main]

extern crate lightning_fuzz;
use lightning_fuzz::msg_targets::msg_funding_signed::*;

use std::io::Read;

#[cfg(feature = "afl")]
#[macro_use] extern crate afl;
#[cfg(feature = "afl")]
fn main() {
	fuzz!(|data| {
		msg_funding_signed_run(data.as_ptr(), data.len());
	});
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
	loop {
		fuzz!(|data| {
			msg_funding_signed_run(data.as_ptr(), data.len());
		});
	}
}

#[cfg(feature = "libfuzzer_fuzz")]
#[macro_use] extern crate libfuzzer_sys;
#[cfg(feature = "libfuzzer_fuzz")]
fuzz_target!(|data: &[u8]| {
	msg_funding_signed_run(data.as_ptr(), data.len());
});

#[cfg(feature = "stdin_fuzz")]
fn main() {
	let mut data = Vec::with_capacity(8192);
	std::io::stdin().read_to_end(&mut data).unwrap();
	msg_funding_signed_run(data.as_ptr(), data.len());
}
