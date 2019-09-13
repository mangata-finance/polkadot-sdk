#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "strict", deny(warnings))]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use rstd::{vec::Vec, slice, vec};

use runtime_io::{
	set_storage, storage, clear_prefix, blake2_128, blake2_256,
	twox_128, twox_256, ed25519_verify, sr25519_verify,
};
use sr_primitives::{print, traits::{BlakeTwo256, Hash}};
use primitives::{ed25519, sr25519};

macro_rules! impl_stubs {
	( $( $new_name:ident => $invoke:expr, )* ) => {
		$(
			impl_stubs!(@METHOD $new_name => $invoke);
		)*
	};
	( @METHOD $new_name:ident => $invoke:expr ) => {
		#[no_mangle]
		pub fn $new_name(input_data: *mut u8, input_len: usize) -> u64 {
			let input: &[u8] = if input_len == 0 {
				&[0u8; 0]
			} else {
				unsafe {
					slice::from_raw_parts(input_data, input_len)
				}
			};

			let output: Vec<u8> = $invoke(input);
			let res = output.as_ptr() as u64 + ((output.len() as u64) << 32);

			// Leak the output vector to avoid it being freed.
			// This is fine in a WASM context since the heap
			// will be discarded after the call.
			rstd::mem::forget(output);
			res
		}
	};
}

impl_stubs!(
	test_data_in => |input| {
		print("set_storage");
		set_storage(b"input", input);

		print("storage");
		let foo = storage(b"foo").unwrap();

		print("set_storage");
		set_storage(b"baz", &foo);

		print("finished!");
		b"all ok!".to_vec()
	},
	test_clear_prefix => |input| {
		clear_prefix(input);
		b"all ok!".to_vec()
	},
	test_empty_return => |_| Vec::new(),
	test_exhaust_heap => |_| Vec::with_capacity(16777216),
	test_panic => |_| panic!("test panic"),
	test_conditional_panic => |input: &[u8]| {
		if input.len() > 0 {
			panic!("test panic")
		}
		input.to_vec()
	},
	test_blake2_256 => |input| blake2_256(input).to_vec(),
	test_blake2_128 => |input| blake2_128(input).to_vec(),
	test_twox_256 => |input| twox_256(input).to_vec(),
	test_twox_128 => |input| twox_128(input).to_vec(),
	test_ed25519_verify => |input: &[u8]| {
		let mut pubkey = [0; 32];
		let mut sig = [0; 64];

		pubkey.copy_from_slice(&input[0..32]);
		sig.copy_from_slice(&input[32..96]);

		let msg = b"all ok!";
		[ed25519_verify(&ed25519::Signature(sig), &msg[..], &ed25519::Public(pubkey)) as u8].to_vec()
	},
	test_sr25519_verify => |input: &[u8]| {
		let mut pubkey = [0; 32];
		let mut sig = [0; 64];

		pubkey.copy_from_slice(&input[0..32]);
		sig.copy_from_slice(&input[32..96]);

		let msg = b"all ok!";
		[sr25519_verify(&sr25519::Signature(sig), &msg[..], &sr25519::Public(pubkey)) as u8].to_vec()
	},
	test_ordered_trie_root => |_| {
		BlakeTwo256::ordered_trie_root(
			vec![
				b"zero"[..].into(),
				b"one"[..].into(),
				b"two"[..].into(),
			],
		).as_ref().to_vec()
	},
	test_sandbox => |code: &[u8]| {
		let ok = execute_sandboxed(code, &[]).is_ok();
		[ok as u8].to_vec()
	},
	test_sandbox_args => |code: &[u8]| {
		let ok = execute_sandboxed(
			code,
			&[
				sandbox::TypedValue::I32(0x12345678),
				sandbox::TypedValue::I64(0x1234567887654321),
			]
		).is_ok();
		[ok as u8].to_vec()
	},
	test_sandbox_return_val => |code: &[u8]| {
		let ok = match execute_sandboxed(
			code,
			&[
				sandbox::TypedValue::I32(0x1336),
			]
		) {
			Ok(sandbox::ReturnValue::Value(sandbox::TypedValue::I32(0x1337))) => true,
			_ => false,
		};
		[ok as u8].to_vec()
	},
	test_sandbox_instantiate => |code: &[u8]| {
		let env_builder = sandbox::EnvironmentDefinitionBuilder::new();
		let code = match sandbox::Instance::new(code, &env_builder, &mut ()) {
			Ok(_) => 0,
			Err(sandbox::Error::Module) => 1,
			Err(sandbox::Error::Execution) => 2,
			Err(sandbox::Error::OutOfBounds) => 3,
		};
		[code].to_vec()
	},
	test_offchain_local_storage => |_| {
		let kind = primitives::offchain::StorageKind::PERSISTENT;
		assert_eq!(runtime_io::local_storage_get(kind, b"test"), None);
		runtime_io::local_storage_set(kind, b"test", b"asd");
		assert_eq!(runtime_io::local_storage_get(kind, b"test"), Some(b"asd".to_vec()));

		let res = runtime_io::local_storage_compare_and_set(kind, b"test", Some(b"asd"), b"");
		assert_eq!(res, true);
		assert_eq!(runtime_io::local_storage_get(kind, b"test"), Some(b"".to_vec()));

		[0].to_vec()
	},
	test_offchain_local_storage_with_none => |_| {
		let kind = primitives::offchain::StorageKind::PERSISTENT;
		assert_eq!(runtime_io::local_storage_get(kind, b"test"), None);

		let res = runtime_io::local_storage_compare_and_set(kind, b"test", None, b"value");
		assert_eq!(res, true);
		assert_eq!(runtime_io::local_storage_get(kind, b"test"), Some(b"value".to_vec()));

		[0].to_vec()
	},
	test_offchain_http => |_| {
		use primitives::offchain::HttpRequestStatus;
		let run = || -> Option<()> {
			let id = runtime_io::http_request_start("POST", "http://localhost:12345", &[]).ok()?;
			runtime_io::http_request_add_header(id, "X-Auth", "test").ok()?;
			runtime_io::http_request_write_body(id, &[1, 2, 3, 4], None).ok()?;
			runtime_io::http_request_write_body(id, &[], None).ok()?;
			let status = runtime_io::http_response_wait(&[id], None);
			assert!(status == vec![HttpRequestStatus::Finished(200)], "Expected Finished(200) status.");
			let headers = runtime_io::http_response_headers(id);
			assert_eq!(headers, vec![(b"X-Auth".to_vec(), b"hello".to_vec())]);
			let mut buffer = vec![0; 64];
			let read = runtime_io::http_response_read_body(id, &mut buffer, None).ok()?;
			assert_eq!(read, 3);
			assert_eq!(&buffer[0..read], &[1, 2, 3]);
			let read = runtime_io::http_response_read_body(id, &mut buffer, None).ok()?;
			assert_eq!(read, 0);

			Some(())
		};

		[if run().is_some() { 0 } else { 1 }].to_vec()
	},
);

fn execute_sandboxed(code: &[u8], args: &[sandbox::TypedValue]) -> Result<sandbox::ReturnValue, sandbox::HostError> {
	struct State {
		counter: u32,
	}

	fn env_assert(_e: &mut State, args: &[sandbox::TypedValue]) -> Result<sandbox::ReturnValue, sandbox::HostError> {
		if args.len() != 1 {
			return Err(sandbox::HostError);
		}
		let condition = args[0].as_i32().ok_or_else(|| sandbox::HostError)?;
		if condition != 0 {
			Ok(sandbox::ReturnValue::Unit)
		} else {
			Err(sandbox::HostError)
		}
	}
	fn env_inc_counter(e: &mut State, args: &[sandbox::TypedValue]) -> Result<sandbox::ReturnValue, sandbox::HostError> {
		if args.len() != 1 {
			return Err(sandbox::HostError);
		}
		let inc_by = args[0].as_i32().ok_or_else(|| sandbox::HostError)?;
		e.counter += inc_by as u32;
		Ok(sandbox::ReturnValue::Value(sandbox::TypedValue::I32(e.counter as i32)))
	}

	let mut state = State { counter: 0 };

	let env_builder = {
		let mut env_builder = sandbox::EnvironmentDefinitionBuilder::new();
		env_builder.add_host_func("env", "assert", env_assert);
		env_builder.add_host_func("env", "inc_counter", env_inc_counter);
		let memory = match sandbox::Memory::new(1, Some(16)) {
			Ok(m) => m,
			Err(_) => unreachable!("
				Memory::new() can return Err only if parameters are borked; \
				We passing params here explicitly and they're correct; \
				Memory::new() can't return a Error qed"
			),
		};
		env_builder.add_memory("env", "memory", memory.clone());
		env_builder
	};

	let mut instance = sandbox::Instance::new(code, &env_builder, &mut state)?;
	let result = instance.invoke(b"call", args, &mut state);

	result.map_err(|_| sandbox::HostError)
}
