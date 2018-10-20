pub mod cuda;

#[cfg(test)]
mod tests {
	use std::os::raw;
	use cuda::*;
	fn vecadd_ptx() -> String {
r#"
.version 4.3
.target sm_20
.address_size 64

        // .globl       addfv

.visible .entry addfv(
        .param .u64 addfv_param_0,
        .param .u64 addfv_param_1,
        .param .u64 addfv_param_2,
        .param .u64 addfv_param_3
)
{
        .reg .pred      %p<2>;
        .reg .f32       %f<4>;
        .reg .b32       %r<5>;
        .reg .b64       %rd<13>;


        ld.param.u64    %rd2, [addfv_param_0];
        ld.param.u64    %rd3, [addfv_param_1];
        ld.param.u64    %rd4, [addfv_param_2];
        ld.param.u64    %rd5, [addfv_param_3];
        mov.u32         %r1, %ntid.x;
        mov.u32         %r2, %ctaid.x;
        mov.u32         %r3, %tid.x;
        mad.lo.s32      %r4, %r2, %r1, %r3;
        cvt.u64.u32     %rd1, %r4;
        setp.ge.u64     %p1, %rd1, %rd5;
        @%p1 bra        BB0_2;

        cvta.to.global.u64      %rd6, %rd2;
        shl.b64         %rd7, %rd1, 2;
        add.s64         %rd8, %rd6, %rd7;
        cvta.to.global.u64      %rd9, %rd3;
        add.s64         %rd10, %rd9, %rd7;
        ld.global.f32   %f1, [%rd10];
        ld.global.f32   %f2, [%rd8];
        add.f32         %f3, %f2, %f1;
        cvta.to.global.u64      %rd11, %rd4;
        add.s64         %rd12, %rd11, %rd7;
        st.global.f32   [%rd12], %f3;

BB0_2:
        ret;
}
"#.to_string()
	}

	macro_rules! cu {
		($call:expr) => ({
			let res: CUresult = $call;
			assert_eq!(res, CUresult::CUDA_SUCCESS);
		})
	}

	#[test]
	fn bringup_teardown() {
		let dev: CUdevice = 0;
		let flags: raw::c_uint = CUctx_flags::CU_CTX_SCHED_AUTO as raw::c_uint;
		unsafe {
			cu!(cuInit(0 as raw::c_uint));
			let mut ctx: CUcontext = ::std::mem::uninitialized();
			cu!(cuCtxCreate_v2(&mut ctx, flags, dev));
			cu!(cuCtxDestroy_v2(ctx));
		}
	}

	struct Context {
		ctx: CUcontext
	}
	impl Context {
		fn new() -> Self {
			let dev: CUdevice = 0;
			let flags: raw::c_uint = CUctx_flags::CU_CTX_SCHED_AUTO as raw::c_uint;
			unsafe {
				let mut ctxt: CUcontext = ::std::mem::uninitialized();
				cu!(cuInit(0 as raw::c_uint));
				cu!(cuCtxCreate_v2(&mut ctxt, flags, dev));
				Context{ctx: ctxt}
			}
		}
	}
	impl Drop for Context {
		fn drop(&mut self) {
			unsafe {
				cu!(cuCtxDestroy_v2(self.ctx));
				self.ctx = ::std::mem::uninitialized();
			}
		}
	}
	#[test]
	fn alloc() {
		let _foo = Context::new();
		unsafe {
			let mut mem = 0 as CUdeviceptr;
			cu!(cuMemAlloc_v2(&mut mem, 64*12*4));
			cu!(cuMemFree_v2(mem));
		}
	}

	// Simple test to load a module and execute the kernel in it.  The add kernel
	// it implements is basically:
	//   fn addfv(a: [f32;n], b: [f32;n], c: &mut [f32;n], n: usize) {
	//     let i = blockDim.x*blockIdx.x + threadIdx.x;
	//     if i >= n { return; }
	//     c[i] = a[i] + b[i];
	//   }
	#[test]
	fn module_run() {
		let _ctx = Context::new();
		unsafe {
			let mut a = 0 as CUdeviceptr;
			let mut b = 0 as CUdeviceptr;
			let mut c = 0 as CUdeviceptr;
			let mut n = 0 as CUdeviceptr;
			let size_flt = ::std::mem::size_of::<f32>();
			cu!(cuMemAlloc_v2(&mut a, 64*12*size_flt));
			cu!(cuMemAlloc_v2(&mut b, 64*12*size_flt));
			cu!(cuMemAlloc_v2(&mut c, 64*12*size_flt));
			let global = CUmemAttach_flags::CU_MEM_ATTACH_GLOBAL as u32;
			cu!(cuMemAllocManaged(&mut n, ::std::mem::size_of::<usize>(), global));
			let n_host = n as *mut usize;
			*n_host = 64*12;
			let mut module: CUmodule = ::std::mem::uninitialized();
			let ptx: String = vecadd_ptx();
			cu!(cuModuleLoadData(&mut module, ptx.as_ptr() as *const raw::c_void));
			let fqnname = "addfv";
			let mut fqn: CUfunction = ::std::mem::uninitialized();
			cu!(cuModuleGetFunction(&mut fqn, module,
			                        fqnname.as_ptr() as *const raw::c_char));
			let mut strm: CUstream = ::std::mem::uninitialized();
			let nonblock = CUstream_flags::CU_STREAM_NON_BLOCKING as u32;
			cu!(cuStreamCreate(&mut strm, nonblock));

			let grid: [raw::c_uint;3] = [4, 1, 1];
			let blk: [raw::c_uint;3] = [16, 12, 1];
			let shmem = 0 as raw::c_uint;
			let null: *mut *mut raw::c_void = 0 as *mut *mut raw::c_void;
			// not actually mutable, just satisfying interface:
			let mut params: [*mut raw::c_void;5] = [
				&a as *const CUdeviceptr as *mut raw::c_void,
				&b as *const CUdeviceptr as *mut raw::c_void,
				&c as *const CUdeviceptr as *mut raw::c_void,
				n as *mut raw::c_void,
				0 as *mut raw::c_void,
			];
			cu!(cuLaunchKernel(fqn, grid[0],grid[1],grid[2], blk[0],blk[1],blk[2],
			                   shmem, strm, params.as_mut_ptr(), null));
			cu!(cuStreamSynchronize(strm));
			cu!(cuStreamDestroy_v2(strm));
			cu!(cuModuleUnload(module));
			cu!(cuMemFree_v2(a));
			cu!(cuMemFree_v2(b));
			cu!(cuMemFree_v2(c));
			cu!(cuMemFree_v2(n));
		}
	}
}
