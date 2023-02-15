//! Example `mexcallmatlab.c` translated to Rust.
use core::ffi;
use matlab_sys::raw::{self, mxArray};
use std::ptr::addr_of_mut;

unsafe fn xr(pa: *mut mxArray, i: usize, j: usize) -> *mut f64 {
    addr_of_mut!((*raw::mxGetComplexDoubles(pa).add(i).add(4 * j)).real)
}
unsafe fn xi(pa: *mut mxArray, i: usize, j: usize) -> *mut f64 {
    addr_of_mut!((*raw::mxGetComplexDoubles(pa).add(i).add(4 * j)).imag)
}

unsafe fn fill_array(pa: *mut mxArray) {
    for j in 0..4 {
        for i in 0..j {
            *xr(pa, i, j) = 4. + i as f64 - j as f64;
            *xr(pa, j, i) = *xr(pa, i, j);
            *xi(pa, i, j) = j as f64 - i as f64 + 1.;
            *xi(pa, j, i) = *xi(pa, i, j);
        }
    }
    /* Reorder columns of xr. */
    for j in 0..2 {
        for i in 0..4 {
            let tmp = *xr(pa, i, j);
            let jj = 3 - j;
            *xr(pa, i, j) = *xr(pa, i, jj);
            *xr(pa, i, jj) = tmp;
        }
    }
}

/* Invert diagonal elements of complex matrix of order 4 */
unsafe fn invertd(pa: *mut mxArray) {
    for i in 0..4 {
        let temp = *xr(pa, i, i) * *xr(pa, i, i) + *xi(pa, i, i) * *xi(pa, i, i);
        *xr(pa, i, i) = *xr(pa, i, i) / temp;
        *xi(pa, i, i) = -*xi(pa, i, i) / temp;
    }
}

#[no_mangle]
pub unsafe extern "C" fn mexFunction(
    nlhs: ffi::c_int,
    plhs: *mut *mut raw::mxArray,
    nrhs: ffi::c_int,
    _prhs: *const *const raw::mxArray,
) {
    let mut lhs: [*mut raw::mxArray; 2] = [std::ptr::null_mut(); 2];

    let m = 4;
    let n = 4;

    /* Check for proper number of input and output arguments */
    if nrhs != 0 {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:mexcallmatlab:maxrhs\0".as_ptr().cast(),
            b"No input arguments required.\0".as_ptr().cast(),
        );
    }
    if nlhs > 1 {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:mexcallmatlab:maxlhs\0".as_ptr().cast(),
            b"Too many output arguments.\0".as_ptr().cast(),
        );
    }

    /* Allocate x matrix */
    let mut x = raw::mxCreateDoubleMatrix(m, n, raw::mxComplexity_mxCOMPLEX);

    /* create values in some arrays -- remember, MATLAB stores matrices
    column-wise */
    fill_array(x);

    /* print out initial matrix */
    raw::mexCallMATLAB(
        0,
        core::ptr::null_mut(),
        1,
        addr_of_mut!(x),
        b"disp\0".as_ptr().cast(),
    );

    /* calculate eigenvalues and eigenvectors */
    raw::mexCallMATLAB(
        2,
        addr_of_mut!(lhs) as _,
        1,
        addr_of_mut!(x),
        b"eig\0".as_ptr().cast(),
    );
    /* print out eigenvalue matrix */
    raw::mexCallMATLAB(
        0,
        core::ptr::null_mut(),
        1,
        addr_of_mut!(lhs[1]),
        b"disp\0".as_ptr().cast(),
    );

    /* take inverse of complex eigenvalues, just on diagonal */
    invertd(lhs[1]);
    /* and print these out */
    raw::mexCallMATLAB(
        0,
        core::ptr::null_mut(),
        1,
        addr_of_mut!(lhs[1]),
        b"disp\0".as_ptr().cast(),
    );

    /* Free allocated matrices */
    raw::mxDestroyArray(x);
    raw::mxDestroyArray(lhs[1]);
    *plhs.add(0) = lhs[0];
}
