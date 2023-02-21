//! Example `arraySize.c` translated to Rust.

use core::ffi;
use matlab_sys::interleaved_complex as raw;

unsafe fn error_check(nlhs: i32, nrhs: i32, prhs: *const *const raw::mxArray) {
    /* Check for proper number of arguments. */
    if nrhs != 1
        || raw::mxIsNumeric(*prhs.add(0)) == false
        || raw::mxGetNumberOfElements(*prhs.add(0)) != 1
    {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:arraySize:rhs\0".as_ptr().cast(),
            b"This function requires one scalar numeric input.\0"
                .as_ptr()
                .cast(),
        );
    }
    let dim = raw::mxGetScalar(*prhs.add(0));
    if dim < 0. {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:arraySize:dimensionNegative\0".as_ptr().cast(),
            b"The input dimension must be positive.\0".as_ptr().cast(),
        );
    }
    if dim as u64 > raw::MWSIZE_MAX {
        raw::mexErrMsgIdAndTxt(b"MATLAB:arraySize:dimensionTooLarge\0".as_ptr().cast(),
        b"The input dimension, %.0f, is larger than the maximum value of mwSize, %u, when built with largeArrayDims.\0".as_ptr().cast(), dim, raw::MWSIZE_MAX);
    }
    if nlhs > 1 {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:arraySize:lhs\0".as_ptr().cast(),
            b"Too many output arguments.\0".as_ptr().cast(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn mexFunction(
    nlhs: ffi::c_int,
    plhs: *mut *mut raw::mxArray,
    nrhs: ffi::c_int,
    prhs: *const *const raw::mxArray,
) {
    error_check(nlhs, nrhs, prhs);
    let dim = raw::mxGetScalar(*prhs.add(0));
    /* Create an mxArray of size dim x dim of type uint8.*/
    let the_array = raw::mxCreateNumericMatrix(
        dim as raw::mwSize,
        dim as raw::mwSize,
        raw::mxClassID_mxUINT8_CLASS,
        raw::mxComplexity_mxREAL,
    );

    /* Display the mxArray's size. */
    let number_of_elements = raw::mxGetNumberOfElements(the_array);
    /* numberOfElements can be converted to a double without loss of
    precision because the maximum size of an array is 2^48-1. */
    let size_of_data_in_kibibytes =
        number_of_elements as f64 * raw::mxGetElementSize(the_array) as f64 / 1024.;
    raw::mexPrintf(
        b"Size of array in kilobytes: %.0f\n\n\0".as_ptr().cast(),
        size_of_data_in_kibibytes,
    );
    /* Return result only if one is requested. */
    if nlhs == 1 {
        *plhs.add(0) = raw::mxCreateDoubleScalar(size_of_data_in_kibibytes);
    }
    raw::mxDestroyArray(the_array);
}
