extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	let cuda_path = PathBuf::from(match env::var("CUDA_PATH") {
		Ok(chome) => chome,
		Err(_) => "/usr/local/cuda".to_string()
	});
	let cuda = match cuda_path.to_str() {
		Some(c) => c,
		None => "cuda-driver-sys: error creating string from cuda path",
	};
	for libdir in vec!["lib64", "lib"] {
		let mut clib_path = cuda_path.clone();
		clib_path.push(libdir);
		// Don't check if the path exists first.  If someone is having issues and
		// turns verbosity on, this will at least clue them in how to hack it.
		println!("cargo:rustc-link-search=native={}/{}", cuda, libdir);
	}
	println!("cargo:rustc-link-lib=cuda"); // link against cuda.

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
			// Tell clang where to find cuda.h.
			.clang_arg(format!("-I{}/include", cuda))
			.header("cuda-driver-sys.h")
			.whitelist_recursively(true) // FIXME set to false
			// Keep these alphabetized.
			.whitelisted_type("CUctx_flags")
			.whitelisted_type("CUdeviceptr")
			.whitelisted_type("CUevent_flags")
			.whitelisted_type("CUfunction_attribute")
			.whitelisted_type("CUmemAttach_flags")
			.whitelisted_type("CUpointer_attribute")
			.whitelisted_type("CUresult")
			.whitelisted_type("CUstream_flags")
			// Keep these alphabetized.
			.whitelisted_function("cuCtxCreate")
			.whitelisted_function("cuCtxCreate_v2")
			.whitelisted_function("cuCtxDestroy")
			.whitelisted_function("cuCtxDestroy_v2")
			.whitelisted_function("cuCtxGetCurrent")
			.whitelisted_function("cuCtxSynchronize")
			.whitelisted_function("cuEventCreate")
			.whitelisted_function("cuEventDestroy")
			.whitelisted_function("cuEventDestroy_v2")
			.whitelisted_function("cuGraphicsMapResources")
			.whitelisted_function("cuGraphicsResourceGetMappedPointer")
			.whitelisted_function("cuGraphicsResourceGetMappedPointer_v2")
			.whitelisted_function("cuGraphicsUnmapResources")
			.whitelisted_function("cuInit")
			.whitelisted_function("cuLaunchKernel")
			.whitelisted_function("cuMemAlloc")
			.whitelisted_function("cuMemAlloc_v2")
			.whitelisted_function("cuMemAllocManaged")
			.whitelisted_function("cuMemAllocPitch")
			.whitelisted_function("cuMemAllocPitch_v2")
			.whitelisted_function("cuMemcpyAsync")
			.whitelisted_function("cuMemcpyPeerAsync")
			.whitelisted_function("cuMemFree")
			.whitelisted_function("cuMemFree_v2")
			.whitelisted_function("cuModuleGetFunction")
			.whitelisted_function("cuModuleLoad")
			.whitelisted_function("cuModuleLoadData")
			.whitelisted_function("cuModuleLoadDataEx")
			.whitelisted_function("cuModuleUnload")
			.whitelisted_function("cuPointerGetAttribute")
			.whitelisted_function("cuPointerGetAttributes")
			.whitelisted_function("cuStreamAttachMemAsync")
			.whitelisted_function("cuStreamCreate")
			.whitelisted_function("cuStreamDestroy")
			.whitelisted_function("cuStreamDestroy_v2")
			.whitelisted_function("cuStreamSynchronize")
			.whitelisted_function("cuStreamWaitEvent")
			.generate()
			.expect("Unable to generate cuda-driver-sys bindings");
	
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("cuda-driver.rs"))
		.expect("Couldn't write cuda-driver-sys bindings!");

	println!("cargo:rerun-if-changed=build.rs");
}
