/* automatically generated by rust-bindgen 0.63.0 */
#![allow(nonstandard_style)]
pub type FILE = _iobuf;
pub type mwSize = usize;
pub type mwIndex = usize;
pub type mwSignedIndex = isize;
#[doc = " Forward declaration for mxArray"]
pub type mxArray = mxArray_tag;
#[doc = " MEX-file entry point type"]
pub type mxFunctionPtr = ::core::option::Option<
    unsafe extern "C" fn(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
    ),
>;
#[doc = " Logical type"]
pub type mxLogical = bool;
#[doc = " Required for Unicode support in MATLAB"]
pub type mxChar = u16;
pub type mxClassID = ::core::ffi::c_int;
#[doc = " Indicates whether floating-point mxArrays are real or complex."]
pub type mxComplexity = ::core::ffi::c_int;
pub type mex_exit_fn = ::core::option::Option<unsafe extern "C" fn()>;
pub type mexGlobalTableEntry = mexGlobalTableEntry_Tag;
pub type mexGlobalTable = *mut mexGlobalTableEntry_Tag;
pub type mexFunctionTableEntry = mexFunctionTableEntry_tag;
pub type mexFunctionTable = *mut mexFunctionTableEntry_tag;
pub type mexLocalFunctionTable = *mut _mexLocalFunctionTable;
pub type mexInitTermTableEntry = *mut _mexInitTermTableEntry;
pub type fn_clean_up_after_error = ::core::option::Option<unsafe extern "C" fn()>;
pub type fn_simple_function_to_string =
    ::core::option::Option<unsafe extern "C" fn(f: mxFunctionPtr) -> *const ::core::ffi::c_char>;
pub type fn_mex_get_local_function_table =
    ::core::option::Option<unsafe extern "C" fn() -> mexLocalFunctionTable>;
pub type fn_mex_set_local_function_table = ::core::option::Option<
    unsafe extern "C" fn(arg1: mexLocalFunctionTable) -> mexLocalFunctionTable,
>;
pub type MATFile = MatFile_tag;
pub type matError = ::core::ffi::c_int;
pub type Engine = engine;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxArray_tag {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxComplexDouble {
    pub real: f64,
    pub imag: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxComplexSingle {
    pub real: f32,
    pub imag: f32,
}
#[repr(C)]
pub struct mxComplexInt8 {
    pub real: i8,
    pub imag: i8,
}
#[repr(C)]
pub struct mxComplexUint8 {
    pub real: u8,
    pub imag: u8,
}
#[repr(C)]
pub struct mxComplexInt16 {
    pub real: i16,
    pub imag: i16,
}
#[repr(C)]
pub struct mxComplexUint16 {
    pub real: u16,
    pub imag: u16,
}
#[repr(C)]
pub struct mxComplexInt32 {
    pub real: i32,
    pub imag: i32,
}
#[repr(C)]
pub struct mxComplexUint32 {
    pub real: u32,
    pub imag: u32,
}
#[repr(C)]
pub struct mxComplexInt64 {
    pub real: i64,
    pub imag: i64,
}
#[repr(C)]
pub struct mxComplexUint64 {
    pub real: u64,
    pub imag: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mexGlobalTableEntry_Tag {
    pub name: *const ::core::ffi::c_char,
    pub variable: *mut *mut mxArray,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mexFunctionTableEntry_tag {
    pub name: *const ::core::ffi::c_char,
    pub f: mxFunctionPtr,
    pub nargin: ::core::ffi::c_int,
    pub nargout: ::core::ffi::c_int,
    pub local_function_table: *mut _mexLocalFunctionTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mexLocalFunctionTable {
    pub length: usize,
    pub entries: mexFunctionTable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mexInitTermTableEntry {
    pub initialize: ::core::option::Option<unsafe extern "C" fn()>,
    pub terminate: ::core::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatFile_tag {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct engine {
    _unused: [u8; 0],
}
pub const MWSIZE_MAX: u64 = 281474976710655;
pub const MWINDEX_MAX: u64 = 281474976710655;
pub const MWSINDEX_MAX: u64 = 281474976710655;
pub const MWSINDEX_MIN: i64 = -281474976710655;
pub const MWSIZE_MIN: u32 = 0;
pub const MWINDEX_MIN: u32 = 0;
pub const MW_FIRST_API_VERSION: u32 = 700;
pub const MW_LATEST_API_VERSION: u32 = 800;
pub const mxMAXNAM: u32 = 64;
pub const mxClassID_mxUNKNOWN_CLASS: mxClassID = 0;
pub const mxClassID_mxCELL_CLASS: mxClassID = 1;
pub const mxClassID_mxSTRUCT_CLASS: mxClassID = 2;
pub const mxClassID_mxLOGICAL_CLASS: mxClassID = 3;
pub const mxClassID_mxCHAR_CLASS: mxClassID = 4;
pub const mxClassID_mxVOID_CLASS: mxClassID = 5;
pub const mxClassID_mxDOUBLE_CLASS: mxClassID = 6;
pub const mxClassID_mxSINGLE_CLASS: mxClassID = 7;
pub const mxClassID_mxINT8_CLASS: mxClassID = 8;
pub const mxClassID_mxUINT8_CLASS: mxClassID = 9;
pub const mxClassID_mxINT16_CLASS: mxClassID = 10;
pub const mxClassID_mxUINT16_CLASS: mxClassID = 11;
pub const mxClassID_mxINT32_CLASS: mxClassID = 12;
pub const mxClassID_mxUINT32_CLASS: mxClassID = 13;
pub const mxClassID_mxINT64_CLASS: mxClassID = 14;
pub const mxClassID_mxUINT64_CLASS: mxClassID = 15;
pub const mxClassID_mxFUNCTION_CLASS: mxClassID = 16;
pub const mxClassID_mxOPAQUE_CLASS: mxClassID = 17;
pub const mxClassID_mxOBJECT_CLASS: mxClassID = 18;
pub const mxClassID_mxINDEX_CLASS: mxClassID = 15;
pub const mxComplexity_mxREAL: mxComplexity = 0;
pub const mxComplexity_mxCOMPLEX: mxComplexity = 1;
extern "C" {
    pub fn mxMalloc_800(n: usize) -> *mut ::core::ffi::c_void;
    pub fn mxCalloc_800(n: usize, size: usize) -> *mut ::core::ffi::c_void;
    pub fn mxFree_800(ptr: *mut ::core::ffi::c_void);
    pub fn mxRealloc_800(ptr: *mut ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void;
    pub fn mxGetNumberOfDimensions_800(pa: *const mxArray) -> mwSize;
    pub fn mxGetDimensions_800(pa: *const mxArray) -> *const mwSize;
    pub fn mxGetM_800(pa: *const mxArray) -> usize;
    pub fn mxGetIr_800(pa: *const mxArray) -> *mut mwIndex;
    pub fn mxGetJc_800(pa: *const mxArray) -> *mut mwIndex;
    pub fn mxGetNzmax_800(pa: *const mxArray) -> mwSize;
    pub fn mxSetNzmax_800(pa: *mut mxArray, nzmax: mwSize);
    pub fn mxGetFieldNameByNumber_800(
        pa: *const mxArray,
        n: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    pub fn mxGetFieldByNumber_800(
        pa: *const mxArray,
        i: mwIndex,
        fieldnum: ::core::ffi::c_int,
    ) -> *mut mxArray;
    pub fn mxGetCell_800(pa: *const mxArray, i: mwIndex) -> *mut mxArray;
    pub fn mxGetClassID_800(pa: *const mxArray) -> mxClassID;
    pub fn mxGetData_800(pa: *const mxArray) -> *mut ::core::ffi::c_void;
    pub fn mxSetData_800(pa: *mut mxArray, newdata: *mut ::core::ffi::c_void);
    pub fn mxIsNumeric_800(pa: *const mxArray) -> bool;
    pub fn mxIsCell_800(pa: *const mxArray) -> bool;
    pub fn mxIsLogical_800(pa: *const mxArray) -> bool;
    pub fn mxIsScalar_800(pa: *const mxArray) -> bool;
    pub fn mxIsChar_800(pa: *const mxArray) -> bool;
    pub fn mxIsStruct_800(pa: *const mxArray) -> bool;
    pub fn mxIsOpaque_800(pa: *const mxArray) -> bool;
    pub fn mxIsFunctionHandle_800(pa: *const mxArray) -> bool;
    pub fn mxIsObject_800(pa: *const mxArray) -> bool;
    pub fn mxIsComplex_800(pa: *const mxArray) -> bool;
    pub fn mxIsSparse_800(pa: *const mxArray) -> bool;
    pub fn mxIsDouble_800(pa: *const mxArray) -> bool;
    pub fn mxIsSingle_800(pa: *const mxArray) -> bool;
    pub fn mxIsInt8_800(pa: *const mxArray) -> bool;
    pub fn mxIsUint8_800(pa: *const mxArray) -> bool;
    pub fn mxIsInt16_800(pa: *const mxArray) -> bool;
    pub fn mxIsUint16_800(pa: *const mxArray) -> bool;
    pub fn mxIsInt32_800(pa: *const mxArray) -> bool;
    pub fn mxIsUint32_800(pa: *const mxArray) -> bool;
    pub fn mxIsInt64_800(pa: *const mxArray) -> bool;
    pub fn mxIsUint64_800(pa: *const mxArray) -> bool;
    pub fn mxGetNumberOfElements_800(pa: *const mxArray) -> usize;
    pub fn mxGetChars_800(pa: *const mxArray) -> *mut mxChar;
    pub fn mxGetUserBits_800(pa: *const mxArray) -> ::core::ffi::c_int;
    pub fn mxSetUserBits_800(pa: *mut mxArray, value: ::core::ffi::c_int);
    pub fn mxGetScalar_800(pa: *const mxArray) -> f64;
    pub fn mxIsFromGlobalWS_800(pa: *const mxArray) -> bool;
    pub fn mxSetFromGlobalWS_800(pa: *mut mxArray, global: bool);
    pub fn mxSetM_800(pa: *mut mxArray, m: mwSize);
    pub fn mxGetN_800(pa: *const mxArray) -> usize;
    pub fn mxIsEmpty_800(pa: *const mxArray) -> bool;
    pub fn mxGetFieldNumber_800(
        pa: *const mxArray,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn mxSetIr_800(pa: *mut mxArray, newir: *mut mwIndex);
    pub fn mxSetJc_800(pa: *mut mxArray, newjc: *mut mwIndex);
    pub fn mxGetPr_800(pa: *const mxArray) -> *mut f64;
    pub fn mxSetPr_800(pa: *mut mxArray, newdata: *mut f64);
    pub fn mxGetElementSize_800(pa: *const mxArray) -> usize;
    pub fn mxCalcSingleSubscript_800(
        pa: *const mxArray,
        nsubs: mwSize,
        subs: *const mwIndex,
    ) -> mwIndex;
    pub fn mxGetNumberOfFields_800(pa: *const mxArray) -> ::core::ffi::c_int;
    pub fn mxSetCell_800(pa: *mut mxArray, i: mwIndex, value: *mut mxArray);
    pub fn mxSetFieldByNumber_800(
        pa: *mut mxArray,
        i: mwIndex,
        fieldnum: ::core::ffi::c_int,
        value: *mut mxArray,
    );
    pub fn mxGetField_800(
        pa: *const mxArray,
        i: mwIndex,
        fieldname: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mxSetField_800(
        pa: *mut mxArray,
        i: mwIndex,
        fieldname: *const ::core::ffi::c_char,
        value: *mut mxArray,
    );
    pub fn mxGetProperty_800(
        pa: *const mxArray,
        i: mwIndex,
        propname: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mxSetProperty_800(
        pa: *mut mxArray,
        i: mwIndex,
        propname: *const ::core::ffi::c_char,
        value: *const mxArray,
    );
    pub fn mxGetClassName_800(pa: *const mxArray) -> *const ::core::ffi::c_char;
    pub fn mxIsClass_800(pa: *const mxArray, name: *const ::core::ffi::c_char) -> bool;
    pub fn mxCreateNumericMatrix_800(
        m: mwSize,
        n: mwSize,
        classid: mxClassID,
        flag: mxComplexity,
    ) -> *mut mxArray;
    pub fn mxCreateUninitNumericMatrix_800(
        m: usize,
        n: usize,
        classid: mxClassID,
        flag: mxComplexity,
    ) -> *mut mxArray;
    pub fn mxCreateUninitNumericArray_800(
        ndim: usize,
        dims: *mut usize,
        classid: mxClassID,
        flag: mxComplexity,
    ) -> *mut mxArray;
    pub fn mxSetN_800(pa: *mut mxArray, n: mwSize);
    pub fn mxSetDimensions_800(
        pa: *mut mxArray,
        pdims: *const mwSize,
        ndims: mwSize,
    ) -> ::core::ffi::c_int;
    pub fn mxDestroyArray_800(pa: *mut mxArray);
    pub fn mxCreateNumericArray_800(
        ndim: mwSize,
        dims: *const mwSize,
        classid: mxClassID,
        flag: mxComplexity,
    ) -> *mut mxArray;
    pub fn mxCreateCharArray_800(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    pub fn mxCreateDoubleMatrix_800(m: mwSize, n: mwSize, flag: mxComplexity) -> *mut mxArray;
    pub fn mxGetLogicals_800(pa: *const mxArray) -> *mut mxLogical;
    pub fn mxCreateLogicalArray_800(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    pub fn mxCreateLogicalMatrix_800(m: mwSize, n: mwSize) -> *mut mxArray;
    pub fn mxCreateLogicalScalar_800(value: bool) -> *mut mxArray;
    pub fn mxIsLogicalScalar_800(pa: *const mxArray) -> bool;
    pub fn mxIsLogicalScalarTrue_800(pa: *const mxArray) -> bool;
    pub fn mxCreateDoubleScalar_800(value: f64) -> *mut mxArray;
    pub fn mxCreateSparse_800(
        m: mwSize,
        n: mwSize,
        nzmax: mwSize,
        flag: mxComplexity,
    ) -> *mut mxArray;
    pub fn mxCreateSparseLogicalMatrix_800(m: mwSize, n: mwSize, nzmax: mwSize) -> *mut mxArray;
    pub fn mxGetNChars_800(pa: *const mxArray, buf: *mut ::core::ffi::c_char, nChars: mwSize);
    pub fn mxGetString_800(
        pa: *const mxArray,
        buf: *mut ::core::ffi::c_char,
        buflen: mwSize,
    ) -> ::core::ffi::c_int;
    pub fn mxArrayToString_800(pa: *const mxArray) -> *mut ::core::ffi::c_char;
    pub fn mxArrayToUTF8String_800(pa: *const mxArray) -> *mut ::core::ffi::c_char;
    pub fn mxCreateStringFromNChars_800(
        str_: *const ::core::ffi::c_char,
        n: mwSize,
    ) -> *mut mxArray;
    pub fn mxCreateString_800(str_: *const ::core::ffi::c_char) -> *mut mxArray;
    pub fn mxCreateCharMatrixFromStrings_800(
        m: mwSize,
        str_: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mxCreateCellMatrix_800(m: mwSize, n: mwSize) -> *mut mxArray;
    pub fn mxCreateCellArray_800(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    pub fn mxCreateStructMatrix_800(
        m: mwSize,
        n: mwSize,
        nfields: ::core::ffi::c_int,
        fieldnames: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mxCreateStructArray_800(
        ndim: mwSize,
        dims: *const mwSize,
        nfields: ::core::ffi::c_int,
        fieldnames: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mxDuplicateArray_800(in_: *const mxArray) -> *mut mxArray;
    pub fn mxSetClassName_800(
        pa: *mut mxArray,
        classname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn mxAddField_800(
        pa: *mut mxArray,
        fieldname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn mxRemoveField_800(pa: *mut mxArray, field: ::core::ffi::c_int);
    pub fn mxGetEps_800() -> f64;
    pub fn mxGetInf_800() -> f64;
    pub fn mxGetNaN_800() -> f64;
    pub fn mxIsFinite_800(x: f64) -> bool;
    pub fn mxIsInf_800(x: f64) -> bool;
    pub fn mxIsNaN_800(x: f64) -> bool;
    pub fn mxGetDoubles_800(arg1: *const mxArray) -> *mut f64;
    pub fn mxSetDoubles_800(arg1: *mut mxArray, arg2: *mut f64) -> ::core::ffi::c_int;
    pub fn mxGetComplexDoubles_800(arg1: *const mxArray) -> *mut mxComplexDouble;
    pub fn mxSetComplexDoubles_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexDouble,
    ) -> ::core::ffi::c_int;
    pub fn mxGetSingles_800(arg1: *const mxArray) -> *mut f32;
    pub fn mxSetSingles_800(arg1: *mut mxArray, arg2: *mut f32) -> ::core::ffi::c_int;
    pub fn mxGetComplexSingles_800(arg1: *const mxArray) -> *mut mxComplexSingle;
    pub fn mxSetComplexSingles_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexSingle,
    ) -> ::core::ffi::c_int;
    pub fn mxGetInt8s_800(arg1: *const mxArray) -> *mut i8;
    pub fn mxSetInt8s_800(arg1: *mut mxArray, arg2: *mut i8) -> ::core::ffi::c_int;
    pub fn mxGetComplexInt8s_800(arg1: *const mxArray) -> *mut mxComplexInt8;
    pub fn mxSetComplexInt8s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexInt8,
    ) -> ::core::ffi::c_int;
    pub fn mxGetUint8s_800(arg1: *const mxArray) -> *mut u8;
    pub fn mxSetUint8s_800(arg1: *mut mxArray, arg2: *mut u8) -> ::core::ffi::c_int;
    pub fn mxGetComplexUint8s_800(arg1: *const mxArray) -> *mut mxComplexUint8;
    pub fn mxSetComplexUint8s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint8,
    ) -> ::core::ffi::c_int;
    pub fn mxGetInt16s_800(arg1: *const mxArray) -> *mut i16;
    pub fn mxSetInt16s_800(arg1: *mut mxArray, arg2: *mut i16) -> ::core::ffi::c_int;
    pub fn mxGetComplexInt16s_800(arg1: *const mxArray) -> *mut mxComplexInt16;
    pub fn mxSetComplexInt16s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexInt16,
    ) -> ::core::ffi::c_int;
    pub fn mxGetUint16s_800(arg1: *const mxArray) -> *mut u16;
    pub fn mxSetUint16s_800(arg1: *mut mxArray, arg2: *mut u16) -> ::core::ffi::c_int;
    pub fn mxGetComplexUint16s_800(arg1: *const mxArray) -> *mut mxComplexUint16;
    pub fn mxSetComplexUint16s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint16,
    ) -> ::core::ffi::c_int;
    pub fn mxGetInt32s_800(arg1: *const mxArray) -> *mut i32;
    pub fn mxSetInt32s_800(arg1: *mut mxArray, arg2: *mut i32) -> ::core::ffi::c_int;
    pub fn mxGetComplexInt32s_800(arg1: *const mxArray) -> *mut mxComplexInt32;
    pub fn mxSetComplexInt32s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexInt32,
    ) -> ::core::ffi::c_int;
    pub fn mxGetUint32s_800(arg1: *const mxArray) -> *mut u32;
    pub fn mxSetUint32s_800(arg1: *mut mxArray, arg2: *mut u32) -> ::core::ffi::c_int;
    pub fn mxGetComplexUint32s_800(arg1: *const mxArray) -> *mut mxComplexUint32;
    pub fn mxSetComplexUint32s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint32,
    ) -> ::core::ffi::c_int;
    pub fn mxGetInt64s_800(arg1: *const mxArray) -> *mut i64;
    pub fn mxSetInt64s_800(arg1: *mut mxArray, arg2: *mut i64) -> ::core::ffi::c_int;
    pub fn mxGetComplexInt64s_800(arg1: *const mxArray) -> *mut mxComplexInt64;
    pub fn mxSetComplexInt64s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexInt64,
    ) -> ::core::ffi::c_int;
    pub fn mxGetUint64s_800(arg1: *const mxArray) -> *mut u64;
    pub fn mxSetUint64s_800(arg1: *mut mxArray, arg2: *mut u64) -> ::core::ffi::c_int;
    pub fn mxGetComplexUint64s_800(arg1: *const mxArray) -> *mut mxComplexUint64;
    pub fn mxSetComplexUint64s_800(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint64,
    ) -> ::core::ffi::c_int;
    pub fn mxMakeArrayReal_800(arg1: *mut mxArray) -> ::core::ffi::c_int;
    pub fn mxMakeArrayComplex_800(arg1: *mut mxArray) -> ::core::ffi::c_int;
    pub fn mexErrMsgTxt_800(error_msg: *const ::core::ffi::c_char);
    pub fn mexErrMsgIdAndTxt_800(
        identifier: *const ::core::ffi::c_char,
        err_msg: *const ::core::ffi::c_char,
        ...
    );
    pub fn mexWarnMsgTxt_800(warn_msg: *const ::core::ffi::c_char);
    pub fn mexWarnMsgIdAndTxt_800(
        identifier: *const ::core::ffi::c_char,
        warn_msg: *const ::core::ffi::c_char,
        ...
    );
    pub fn mexPrintf_800(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn mexMakeArrayPersistent_800(pa: *mut mxArray);
    pub fn mexMakeMemoryPersistent_800(ptr: *mut ::core::ffi::c_void);
    pub fn mexCallMATLAB_800(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
        fcn_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn mexCallMATLABWithTrap_800(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
        fcn_name: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mexPrintAssertion_800(
        test: *const ::core::ffi::c_char,
        fname: *const ::core::ffi::c_char,
        linenum: ::core::ffi::c_int,
        message: *const ::core::ffi::c_char,
    );
    pub fn mexIsGlobal_800(pA: *const mxArray) -> bool;
    pub fn mexPutVariable_800(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        parray: *const mxArray,
    ) -> ::core::ffi::c_int;
    pub fn mexGetVariablePtr_800(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> *const mxArray;
    pub fn mexGetVariable_800(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn mexLock_800();
    pub fn mexUnlock_800();
    pub fn mexIsLocked_800() -> bool;
    pub fn mexFunctionName_800() -> *const ::core::ffi::c_char;
    pub fn mexEvalString_800(str_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn mexEvalStringWithTrap_800(str_: *const ::core::ffi::c_char) -> *mut mxArray;
    pub fn mexAtExit_800(exit_fcn: mex_exit_fn) -> ::core::ffi::c_int;
    pub fn mexFunction(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *const mxArray,
    );
    pub fn matOpen_800(
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut MATFile;
    pub fn matClose_800(pMF: *mut MATFile) -> matError;
    pub fn matGetErrno_800(pMF: *mut MATFile) -> matError;
    pub fn matGetFp_800(pMF: *mut MATFile) -> *mut FILE;
    pub fn matPutVariable_800(
        pMF: *mut MATFile,
        name: *const ::core::ffi::c_char,
        pA: *const mxArray,
    ) -> matError;
    pub fn matPutVariableAsGlobal_800(
        pMF: *mut MATFile,
        name: *const ::core::ffi::c_char,
        pA: *const mxArray,
    ) -> matError;
    pub fn matGetVariable_800(pMF: *mut MATFile, name: *const ::core::ffi::c_char) -> *mut mxArray;
    pub fn matGetNextVariable_800(
        pMF: *mut MATFile,
        nameptr: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn matGetNextVariableInfo_800(
        pMF: *mut MATFile,
        nameptr: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn matGetVariableInfo_800(
        pMF: *mut MATFile,
        name: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    pub fn matDeleteVariable_800(pMF: *mut MATFile, name: *const ::core::ffi::c_char) -> matError;
    pub fn matGetDir_800(
        pMF: *mut MATFile,
        num: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
    pub fn engEvalString(ep: *mut Engine, string: *const ::core::ffi::c_char)
        -> ::core::ffi::c_int;
    pub fn engOpenSingleUse(
        startcmd: *const ::core::ffi::c_char,
        reserved: *mut ::core::ffi::c_void,
        retstatus: *mut ::core::ffi::c_int,
    ) -> *mut Engine;
    pub fn engSetVisible(ep: *mut Engine, newVal: bool) -> ::core::ffi::c_int;
    pub fn engGetVisible(ep: *mut Engine, bVal: *mut bool) -> ::core::ffi::c_int;
    pub fn engOpen(startcmd: *const ::core::ffi::c_char) -> *mut Engine;
    pub fn engClose(ep: *mut Engine) -> ::core::ffi::c_int;
    pub fn engGetVariable(ep: *mut Engine, name: *const ::core::ffi::c_char) -> *mut mxArray;
    pub fn engPutVariable(
        ep: *mut Engine,
        var_name: *const ::core::ffi::c_char,
        ap: *const mxArray,
    ) -> ::core::ffi::c_int;
    pub fn engOutputBuffer(
        ep: *mut Engine,
        buffer: *mut ::core::ffi::c_char,
        buflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
