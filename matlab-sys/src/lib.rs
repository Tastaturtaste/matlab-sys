/*!
 * This library exposes the raw Matlab C-API analogous to the one exposed by the header files 'mex.h', 'matrix.h' and 'mat.h' in C. Depending on the selected features for the representation of complex numbers some functions will change how they interpret their arguments and some Data Types change how complex numbers are stored. These differences can be looked up in the [regular documentation provided by Matlab](https://de.mathworks.com/help/matlab/matlab_external/matlab-support-for-interleaved-complex.html).
 */

#[cfg(not(any(feature = "separate-complex", feature = "interleaved-complex")))]
compile_error!("Either the separate-complex or interleaved-complex feature has to be set");

pub mod raw;
