#[cfg(feature = "separate-complex")]
mod separate_complex_impl;
/// Reexports for the separate-complex API. The differences between the API versions are documented by Mathworks [here][1].
///
/// [1]: https://de.mathworks.com/help/matlab/matlab_external/matlab-support-for-interleaved-complex.html
#[cfg(feature = "separate-complex")]
pub mod separate_complex {
    // While reexporting all items with an API version suffix have this suffix stripped.
    // This matches the behavior in the C headers, which also mask the names with the suffix-less names using macros.
    pub use super::separate_complex_impl::{
        engClose, engEvalString, engGetVariable, engGetVisible, engOpen, engOpenSingleUse,
        engOutputBuffer, engPutVariable, engSetVisible, matClose, matDeleteVariable, matError,
        matGetDir, matGetErrno, matGetFp, matGetNextVariable, matGetNextVariableInfo,
        matGetVariable, matGetVariableInfo, matOpen, matPutVariable, matPutVariableAsGlobal,
        mexAtExit, mexCallMATLABWithObject, mexCallMATLABWithTrapWithObject, mexErrMsgIdAndTxt,
        mexErrMsgTxt, mexEvalString, mexEvalStringWithTrap, mexFunction, mexFunctionName,
        mexGetVariablePtr, mexGetVariableWithObject, mexIsGlobal, mexIsLocked, mexLock,
        mexMakeArrayPersistent, mexMakeMemoryPersistent, mexPrintAssertion, mexPrintf,
        mexPutVariable, mexUnlock, mexWarnMsgIdAndTxt, mexWarnMsgTxt, mex_exit_fn, mwIndex,
        mwSignedIndex, mwSize, mxAddField, mxArray, mxArrayToString, mxArrayToUTF8String,
        mxCalcSingleSubscript, mxCalloc, mxChar, mxClassID, mxComplexity, mxCreateCellArray,
        mxCreateCellMatrix, mxCreateCharArray, mxCreateCharMatrixFromStrings, mxCreateDoubleMatrix,
        mxCreateDoubleScalar, mxCreateLogicalArray, mxCreateLogicalMatrix, mxCreateLogicalScalar,
        mxCreateNumericArray, mxCreateNumericMatrix, mxCreateSharedDataCopy, mxCreateSparse,
        mxCreateSparseLogicalMatrix, mxCreateString, mxCreateStringFromNChars, mxCreateStructArray,
        mxCreateStructMatrix, mxCreateUninitDoubleMatrix, mxCreateUninitNumericArray,
        mxCreateUninitNumericMatrix, mxDestroyArray, mxDuplicateArray, mxFastZeros, mxFree,
        mxFunctionPtr, mxGetCell, mxGetChars, mxGetClassID, mxGetClassName, mxGetData,
        mxGetDimensions, mxGetElementSize, mxGetEps, mxGetField, mxGetFieldByNumber,
        mxGetFieldNameByNumber, mxGetFieldNumber, mxGetImagData, mxGetInf, mxGetIr, mxGetJc,
        mxGetLogicals, mxGetM, mxGetN, mxGetNChars, mxGetNaN, mxGetNumberOfDimensions,
        mxGetNumberOfElements, mxGetNumberOfFields, mxGetNzmax, mxGetPi, mxGetPr, mxGetProperty,
        mxGetPropertyShared, mxGetScalar, mxGetString, mxGetUserBits, mxIsCell, mxIsChar,
        mxIsClass, mxIsComplex, mxIsDouble, mxIsEmpty, mxIsFinite, mxIsFromGlobalWS,
        mxIsFunctionHandle, mxIsInf, mxIsInt16, mxIsInt32, mxIsInt64, mxIsInt8, mxIsLogical,
        mxIsLogicalScalar, mxIsLogicalScalarTrue, mxIsNaN, mxIsNumeric, mxIsObject, mxIsOpaque,
        mxIsScalar, mxIsSingle, mxIsSparse, mxIsStruct, mxIsUint16, mxIsUint32, mxIsUint64,
        mxIsUint8, mxLogical, mxMAXNAM, mxMalloc, mxRealloc, mxRemoveField, mxSetCell,
        mxSetClassName, mxSetData, mxSetDimensions, mxSetField, mxSetFieldByNumber,
        mxSetFromGlobalWS, mxSetImagData, mxSetIr, mxSetJc, mxSetM, mxSetN, mxSetNzmax, mxSetPi,
        mxSetPr, mxSetProperty, mxSetPropertyShared, mxSetUserBits, mxUnreference, mxUnshareArray,
        Engine, MATFile, FILE, MWINDEX_MAX, MWINDEX_MIN, MWSINDEX_MAX, MWSINDEX_MIN, MWSIZE_MAX,
        MWSIZE_MIN,
    };
}
#[cfg(feature = "interleaved-complex")]
mod interleaved_complex_impl;
/// Reexports for the interleaved-complex API. The differences between the API versions are documented by Mathworks [here][1].
///
/// [1]: https://de.mathworks.com/help/matlab/matlab_external/matlab-support-for-interleaved-complex.html
#[cfg(feature = "interleaved-complex")]
pub mod interleaved_complex {
    // While reexporting all items with an API version suffix have this suffix stripped.
    // This matches the behavior in the C headers, which also mask the names with the suffix-less names using macros.
    pub use super::interleaved_complex_impl::{
        engClose, engEvalString, engGetVariable, engGetVisible, engOpen, engOpenSingleUse,
        engOutputBuffer, engPutVariable, engSetVisible, matClose, matDeleteVariable, matError,
        matGetDir, matGetErrno, matGetFp, matGetNextVariable, matGetNextVariableInfo,
        matGetVariable, matGetVariableInfo, matOpen, matPutVariable, matPutVariableAsGlobal,
        mexAtExit, mexCallMATLAB, mexCallMATLABWithTrap, mexErrMsgIdAndTxt, mexErrMsgTxt,
        mexEvalString, mexEvalStringWithTrap, mexFunction, mexFunctionName, mexGetVariable,
        mexGetVariablePtr, mexIsGlobal, mexIsLocked, mexLock, mexMakeArrayPersistent,
        mexMakeMemoryPersistent, mexPrintAssertion, mexPrintf, mexPutVariable, mexUnlock,
        mexWarnMsgIdAndTxt, mexWarnMsgTxt, mex_exit_fn, mwIndex, mwSignedIndex, mwSize, mxAddField,
        mxArray, mxArrayToString, mxArrayToUTF8String, mxCalcSingleSubscript, mxCalloc, mxChar,
        mxClassID, mxComplexDouble, mxComplexInt16, mxComplexInt32, mxComplexInt64, mxComplexInt8,
        mxComplexSingle, mxComplexUint16, mxComplexUint32, mxComplexUint64, mxComplexUint8,
        mxComplexity, mxCreateCellArray, mxCreateCellMatrix, mxCreateCharArray,
        mxCreateCharMatrixFromStrings, mxCreateDoubleMatrix, mxCreateDoubleScalar,
        mxCreateLogicalArray, mxCreateLogicalMatrix, mxCreateLogicalScalar, mxCreateNumericArray,
        mxCreateNumericMatrix, mxCreateSparse, mxCreateSparseLogicalMatrix, mxCreateString,
        mxCreateStringFromNChars, mxCreateStructArray, mxCreateStructMatrix,
        mxCreateUninitNumericArray, mxCreateUninitNumericMatrix, mxDestroyArray, mxDuplicateArray,
        mxFree, mxFunctionPtr, mxGetCell, mxGetChars, mxGetClassID, mxGetClassName,
        mxGetComplexDoubles, mxGetComplexInt16s, mxGetComplexInt32s, mxGetComplexInt64s,
        mxGetComplexInt8s, mxGetComplexSingles, mxGetComplexUint16s, mxGetComplexUint32s,
        mxGetComplexUint64s, mxGetComplexUint8s, mxGetData, mxGetDimensions, mxGetDoubles,
        mxGetElementSize, mxGetEps, mxGetField, mxGetFieldByNumber, mxGetFieldNameByNumber,
        mxGetFieldNumber, mxGetInf, mxGetInt16s, mxGetInt32s, mxGetInt64s, mxGetInt8s, mxGetIr,
        mxGetJc, mxGetLogicals, mxGetM, mxGetN, mxGetNChars, mxGetNaN, mxGetNumberOfDimensions,
        mxGetNumberOfElements, mxGetNumberOfFields, mxGetNzmax, mxGetPr, mxGetProperty,
        mxGetScalar, mxGetSingles, mxGetString, mxGetUint16s, mxGetUint32s, mxGetUint64s,
        mxGetUint8s, mxGetUserBits, mxIsCell, mxIsChar, mxIsClass, mxIsComplex, mxIsDouble,
        mxIsEmpty, mxIsFinite, mxIsFromGlobalWS, mxIsFunctionHandle, mxIsInf, mxIsInt16, mxIsInt32,
        mxIsInt64, mxIsInt8, mxIsLogical, mxIsLogicalScalar, mxIsLogicalScalarTrue, mxIsNaN,
        mxIsNumeric, mxIsObject, mxIsOpaque, mxIsScalar, mxIsSingle, mxIsSparse, mxIsStruct,
        mxIsUint16, mxIsUint32, mxIsUint64, mxIsUint8, mxLogical, mxMAXNAM, mxMakeArrayComplex,
        mxMakeArrayReal, mxMalloc, mxRealloc, mxRemoveField, mxSetCell, mxSetClassName,
        mxSetComplexDoubles, mxSetComplexInt16s, mxSetComplexInt32s, mxSetComplexInt64s,
        mxSetComplexInt8s, mxSetComplexSingles, mxSetComplexUint16s, mxSetComplexUint32s,
        mxSetComplexUint64s, mxSetComplexUint8s, mxSetData, mxSetDimensions, mxSetDoubles,
        mxSetField, mxSetFieldByNumber, mxSetFromGlobalWS, mxSetInt16s, mxSetInt32s, mxSetInt64s,
        mxSetInt8s, mxSetIr, mxSetJc, mxSetM, mxSetN, mxSetNzmax, mxSetPr, mxSetProperty,
        mxSetSingles, mxSetUint16s, mxSetUint32s, mxSetUint64s, mxSetUint8s, mxSetUserBits, Engine,
        MATFile, FILE, MWINDEX_MAX, MWINDEX_MIN, MWSINDEX_MAX, MWSINDEX_MIN, MWSIZE_MAX,
        MWSIZE_MIN,
    };
}
