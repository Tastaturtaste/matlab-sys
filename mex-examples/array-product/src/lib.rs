//! This example is intended to show how to convert a mex function from C to Rust. To this end the [arrayProduct](https://de.mathworks.com/help/matlab/matlab_external/standalone-example.html) example is converted verbatim into Rust using the matlab-sys crate.

// Allow non-snake-case names for this example to stay as close to the C-Code as possible for comparability.
#![allow(non_snake_case)]
use matlab_sys::interleaved_complex as raw;
use std::ffi::{c_int, CString};

fn mexErrMsgIdAndTxt(identifier: &str, err_msg: &str) {
    unsafe {
        // rustc warns against the usage of pointers to temporary CStrings since they get destructed at the end of the statement and usually this is a problem. In this case the function must return first for the statement to end, which means the pointers to the temporary strings cannot dangle.
        #[allow(temporary_cstring_as_ptr)]
        raw::mexErrMsgIdAndTxt(
            CString::new(identifier)
                .expect("The passed string slice should not have internal 0 bytes")
                .as_ptr(),
            CString::new(err_msg)
                .expect("The passed string slice should not have internal 0 bytes")
                .as_ptr(),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn mexFunction(
    nlhs: c_int,
    plhs: *mut *mut raw::mxArray,
    nrhs: c_int,
    prhs: *const *const raw::mxArray,
) {
    if nrhs != 2 {
        mexErrMsgIdAndTxt("MyToolbox:arrayProduct:nrhs", "Two inputs required.");
    }
    if nlhs != 1 {
        mexErrMsgIdAndTxt("MyToolbox:arrayProduct:nlhs", "One output required.");
    }
    // SAFETY: nlhs and nrhs are verified above to be greater or equal to 0 and it's the users responsibility to make sure the passed pointers are valid.
    let plhs = std::slice::from_raw_parts_mut(plhs, nlhs as usize);
    let prhs = std::slice::from_raw_parts(prhs, nrhs as usize);

    /* make sure the first input argument is scalar */
    if !raw::mxIsDouble(prhs[0])
        || raw::mxIsComplex(prhs[0])
        || raw::mxGetNumberOfElements(prhs[0]) != 1
    {
        mexErrMsgIdAndTxt(
            "MyToolbox:arrayProduct:notScalar",
            "Input multiplier must be a scalar.",
        );
    }

    if !raw::mxIsDouble(prhs[1]) || raw::mxIsComplex(prhs[1]) {
        mexErrMsgIdAndTxt(
            "MyToolbox:arrayProduct:notDouble",
            "Input matrix must be type double.",
        );
    }

    /* check that number of rows in second input argument is 1 */
    if raw::mxGetM(prhs[1]) != 1 {
        mexErrMsgIdAndTxt(
            "MyToolbox:arrayProduct:notRowVector",
            "Input must be a row vector.",
        );
    }

    /* get the value of the scalar input  */
    let multiplier = raw::mxGetScalar(prhs[0]);

    /* create a pointer to the real data in the input matrix  */
    let inMatrix = raw::mxGetDoubles(prhs[1]);

    /* get dimensions of the input matrix */
    let ncols = raw::mxGetN(prhs[1]);

    /* create the output matrix */
    plhs[0] = raw::mxCreateDoubleMatrix(1, ncols, raw::mxComplexity::mxREAL);

    /* get a pointer to the real data in the output matrix */
    let outMatrix = raw::mxGetDoubles(plhs[0]);

    /* call the computational routine */
    array_product(multiplier, inMatrix, outMatrix, ncols);
}

unsafe fn array_product(x: f64, y: *mut f64, z: *mut f64, n: usize) {
    unsafe {
        for i in 0..n {
            *z.add(i) = x * *y.add(i);
        }
    }
}
