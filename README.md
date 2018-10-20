These are raw Rust bindings to the CUDA driver API.  See:

  https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html

Note this is only the *driver* API. As such, this will only force your program
to link against libcuda (not libcudart).  The runtime API will never be exposed
through this package.
