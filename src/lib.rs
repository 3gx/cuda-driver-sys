pub mod cuda;

#[cfg(test)]
mod tests {
	use std::os::raw;
	use cuda::*;

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
			let mut mem: CUdeviceptr = ::std::mem::uninitialized();
			cu!(cuMemAlloc_v2(&mut mem, 64*12*4));
			cu!(cuMemFree_v2(mem));
		}
	}
}
