# matlab-sys
matlab-sys provides low level bindings to Matlabs C API to allow writing [MEX functions](https://de.mathworks.com/help/matlab/call-mex-files-1.html?s_tid=CRUX_lftnav) in Rust while using the [C Matrix API](https://de.mathworks.com/help/matlab/cc-mx-matrix-library.html?s_tid=CRUX_lftnav) and interacting with the [C MAT API](https://de.mathworks.com/help/matlab/matlab-c-api-to-read-mat-file-data.html?s_tid=CRUX_lftnav).  

## Usage
To make use of the functionality provided by this crate it has to be linked against the library files 'libmex.lib', 'libmx.lib' and 'libmat.lib' provided by your installation of Matlab. If the environment variable `MATLABPATH` is set to the installation directory of Matlab to link against this path will be used preferentially. Otherwise the build script will attempt to seek the installation directory of the default matlab installation by running the following command on the build machine: `matlab -batch "disp(matlabroot)"`. This usually takes a few seconds for Matlab to start up, so explicitly providing the path in the aforementioned environment variable helps compile times and should be preferred.

To build a mex function in Rust the crate type has to be set to `cdylib`, so a dynamic link library. The entry point for a mex function is a function with the signature 
```
#[no_mangle]
pub unsafe extern "C" fn mexFunction(
    nlhs: c_int,
    plhs: *mut *mut mxArray,
    nrhs: c_int,
    prhs: *const *const mxArray,
)
```
For an example of how to write a mex function using this crate look into the mex-examples directory. 
After building the build artifact can simply be renamed into a *.mexw64 for windows or a *.mexa64 for linux, where the filename is the name of the function callable in Matlab.

## Features
### separate-complex  
This feature enables the separate complex storage representation for complex numbers, which is the only available option for Matlab versions 2017b and prior. Since Matlab internally as well as many mathematical libraries use an interleaved complex representation this can incur unnessecary copying.  
[Complex Storage Documentation]
### interleaved-complex - default  
This feature enables the interleaved complex storage representation for complex numbers, which is available since Matlab 2018a Update 3. Since many mathematical libraries use an interleaved complex representation this can potentially help avoid copies. This API is slated to become the default for Matlabs mex command as well.  
[Complex Storage Documentation]
### link-lib - default  
This feature enables automatically linking to the necessary libraries in your Matlab installation. If you need more control over this process you can disable this feature by disabling the default features.



[Complex Storage Documentation]: https://de.mathworks.com/help/matlab/matlab_external/matlab-support-for-interleaved-complex.html