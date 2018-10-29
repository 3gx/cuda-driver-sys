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

	// The general rules on what to whitelist:
	//   - never anything from the runtime API,
	//   - nothing concerning implicit state (i.e. no CU_STREAM_PER_THREAD),
	//   - debatably, nothing that is marked deprecated,
	//   - unquestioningly, nothing that is deprecated in CUDA 10.0 or earlier.
	let bindings = bindgen::Builder::default()
			// Tell clang where to find cuda.h.
			.clang_arg(format!("-I{}/include", cuda))
			.header("cuda-driver-sys.h")
			.layout_tests(false)
			// prepend_enum_name and constified_enum together make it so that the
			// bindings create a constant for e.g. "CU_CTX_SCHED_AUTO", instead of
			// something that needs to be qualified (a la
			// "CUctx_flags::CU_CTX_SCHED_AUTO").
			.prepend_enum_name(false)
			.constified_enum("CU[A-Za-z0-9_]+_enum")
			.whitelist_recursively(false)
			.whitelisted_type("cuuint[0-9]+_t")
			.whitelisted_type("cudaError_enum")
			.whitelisted_type("CU[A-Za-z0-9_]+_enum")
			.whitelisted_type("CU[A-Za-z0-9_]+_st")
			// Keep these alphabetized.
			.whitelisted_type("CUaddress_mode")
			.whitelisted_type("CUarray")
			.whitelisted_type("CUarray_cubemap_face")
			.whitelisted_type("CUarray_format")
			.whitelisted_type("CUcomputemode")
			.whitelisted_type("CUcontext")
			.whitelisted_type("CUctx_flags")
			.whitelisted_type("CUDA_ARRAY_DESCRIPTOR")
			.whitelisted_type("CUDA_ARRAY3D_DESCRIPTOR")
			.whitelisted_type("CUDA_MEMCPY2D")
			.whitelisted_type("CUDA_MEMCPY3D")
			.whitelisted_type("CUDA_MEMCPY3D_PEER")
			.whitelisted_type("CUDA_RESOURCE_DESC")
			.whitelisted_type("CUDA_RESOURCE_DESC_st")
			.whitelisted_type("CUDA_TEXTURE_DESC")
			.whitelisted_type("CUDA_TEXTURE_DESC_st")
			.whitelisted_type("CUdevice")
			.whitelisted_type("CUdeviceptr")
			.whitelisted_type("CUdevice_attribute")
			.whitelisted_type("CUexternalMemoryHandleType")
			.whitelisted_type("CUexternalSemaphoreHandleType")
			.whitelisted_type("CUevent")
			.whitelisted_type("CUevent_flags")
			.whitelisted_type("CUfilter_mode")
			.whitelisted_type("CUfunction")
			.whitelisted_type("CUfunction_attribute")
			.whitelisted_type("CUfunc_cache")
			.whitelisted_type("CUgraphicsMapResourceFlags")
			.whitelisted_type("CUgraphicsResource")
			.whitelisted_type("CUgraphicsRegisterFlags")
			.whitelisted_type("CUhostFn")
			.whitelisted_type("CUipcEventHandle")
			.whitelisted_type("CUipcMemHandle")
			.whitelisted_type("CUipcMem_flags")
			.whitelisted_type("CUjitInputType")
			.whitelisted_type("CUjit_option")
			.whitelisted_type("CUjit_target")
			.whitelisted_type("CUlimit")
			.whitelisted_type("CUmemAttach_flags")
			.whitelisted_type("CUmemorytype")
			.whitelisted_type("CUmipmappedArray")
			.whitelisted_type("CUmodule")
			.whitelisted_type("CUpointer_attribute")
			.whitelisted_type("CUresourcetype")
			.whitelisted_type("CUresourceViewFormat")
			.whitelisted_type("CUresult")
			.whitelisted_type("CUsharedconfig")
			.whitelisted_type("CUstream")
			.whitelisted_type("CUstreamBatchMemOpType")
			.whitelisted_type("CUstream_flags")
			// no texture/surface references; use texObjs, surfObjs instead.
			.whitelisted_type("CUsurfObject")
			.whitelisted_type("CUtexObject")
			.whitelisted_type("CUuuid")
			// Keep these alphabetized.
			.whitelisted_function("cuCtxCreate")
			.whitelisted_function("cuCtxCreate_v2")
			.whitelisted_function("cuCtxDestroy")
			.whitelisted_function("cuCtxDestroy_v2")
			.whitelisted_function("cuCtxGetApiVersion")
			.whitelisted_function("cuCtxGetCacheConfig")
			.whitelisted_function("cuCtxGetCurrent")
			.whitelisted_function("cuCtxGetDevice")
			.whitelisted_function("cuCtxGetFlags")
			.whitelisted_function("cuCtxGetLimit")
			.whitelisted_function("cuCtxGetSharedMemConfig")
			.whitelisted_function("cuCtxGetStreamPriorityRange")
			.whitelisted_function("cuCtxPushCurrent")
			.whitelisted_function("cuCtxPopCurrent")
			.whitelisted_function("cuCtxSetCacheConfig")
			.whitelisted_function("cuCtxSetSharedMemConfig")
			.whitelisted_function("cuCtxSetCurrent")
			.whitelisted_function("cuCtxSetLimit")
			.whitelisted_function("cuCtxSynchronize")
			// NO *PrimaryCtx* functions!
			.whitelisted_function("cuDeviceGet")
			.whitelisted_function("cuDeviceGetAttribute")
			.whitelisted_function("cuDeviceGetCount")
			.whitelisted_function("cuDeviceGetName")
			.whitelisted_function("cuDeviceTotalMem")
			.whitelisted_function("cuDriverGetVersion")
			.whitelisted_function("cuEventCreate")
			.whitelisted_function("cuEventDestroy")
			.whitelisted_function("cuEventDestroy_v2")
			.whitelisted_function("cuGetErrorName")
			.whitelisted_function("cuGetErrorString")
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
