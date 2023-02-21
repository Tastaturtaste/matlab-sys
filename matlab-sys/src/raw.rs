
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
        engOutputBuffer, engPutVariable, engSetVisible, fn_clean_up_after_error,
        fn_mex_get_local_function_table, fn_mex_set_local_function_table,
        fn_simple_function_to_string, matClose, matDeleteVariable, matError, matGetDir,
        matGetErrno, matGetFp, matGetNextVariable, matGetNextVariableInfo, matGetVariable,
        matGetVariableInfo, matOpen, matPutVariable, matPutVariableAsGlobal, mexAtExit,
        mexCallMATLABWithObject, mexCallMATLABWithTrapWithObject, mexErrMsgIdAndTxt, mexErrMsgTxt,
        mexEvalString, mexEvalStringWithTrap, mexFunction, mexFunctionName, mexFunctionTable,
        mexFunctionTableEntry, mexGetVariablePtr, mexGetVariableWithObject, mexGlobalTable,
        mexGlobalTableEntry, mexInitTermTableEntry, mexIsGlobal, mexIsLocked,
        mexLocalFunctionTable, mexLock, mexMakeArrayPersistent, mexMakeMemoryPersistent,
        mexPrintAssertion, mexPrintf, mexPutVariable, mexUnlock, mexWarnMsgIdAndTxt, mexWarnMsgTxt,
        mex_exit_fn, mwIndex, mwSignedIndex, mwSize, mxAddField, mxArray, mxArrayToString,
        mxArrayToUTF8String, mxCalcSingleSubscript_730 as mxCalcSingleSubscript, mxCalloc, mxChar,
        mxClassID, mxClassID_mxCELL_CLASS, mxClassID_mxCHAR_CLASS, mxClassID_mxDOUBLE_CLASS,
        mxClassID_mxFUNCTION_CLASS, mxClassID_mxINDEX_CLASS, mxClassID_mxINT16_CLASS,
        mxClassID_mxINT32_CLASS, mxClassID_mxINT64_CLASS, mxClassID_mxINT8_CLASS,
        mxClassID_mxLOGICAL_CLASS, mxClassID_mxOBJECT_CLASS, mxClassID_mxOPAQUE_CLASS,
        mxClassID_mxSINGLE_CLASS, mxClassID_mxSTRUCT_CLASS, mxClassID_mxUINT16_CLASS,
        mxClassID_mxUINT32_CLASS, mxClassID_mxUINT64_CLASS, mxClassID_mxUINT8_CLASS,
        mxClassID_mxUNKNOWN_CLASS, mxClassID_mxVOID_CLASS, mxComplexity, mxComplexity_mxCOMPLEX,
        mxComplexity_mxREAL, mxCreateCellArray_730 as mxCreateCellArray,
        mxCreateCellMatrix_730 as mxCreateCellMatrix, mxCreateCharArray_730 as mxCreateCharArray,
        mxCreateCharMatrixFromStrings_730 as mxCreateCharMatrixFromStrings,
        mxCreateDoubleMatrix_730 as mxCreateDoubleMatrix, mxCreateDoubleScalar,
        mxCreateLogicalArray_730 as mxCreateLogicalArray,
        mxCreateLogicalMatrix_730 as mxCreateLogicalMatrix, mxCreateLogicalScalar,
        mxCreateNumericArray_730 as mxCreateNumericArray,
        mxCreateNumericMatrix_730 as mxCreateNumericMatrix, mxCreateSharedDataCopy,
        mxCreateSparseLogicalMatrix_730 as mxCreateSparseLogicalMatrix,
        mxCreateSparse_730 as mxCreateSparse, mxCreateString,
        mxCreateStringFromNChars_730 as mxCreateStringFromNChars,
        mxCreateStructArray_730 as mxCreateStructArray,
        mxCreateStructMatrix_730 as mxCreateStructMatrix, mxCreateUninitDoubleMatrix,
        mxCreateUninitNumericArray, mxCreateUninitNumericMatrix, mxDestroyArray, mxDuplicateArray,
        mxFastZeros, mxFree, mxFunctionPtr, mxGetCell_730 as mxGetCell, mxGetChars, mxGetClassID,
        mxGetClassName, mxGetData, mxGetDimensions_730 as mxGetDimensions, mxGetElementSize,
        mxGetEps, mxGetFieldByNumber_730 as mxGetFieldByNumber, mxGetFieldNameByNumber,
        mxGetFieldNumber, mxGetField_730 as mxGetField, mxGetImagData, mxGetInf,
        mxGetIr_730 as mxGetIr, mxGetJc_730 as mxGetJc, mxGetLogicals, mxGetM, mxGetN,
        mxGetNChars_730 as mxGetNChars, mxGetNaN,
        mxGetNumberOfDimensions_730 as mxGetNumberOfDimensions, mxGetNumberOfElements,
        mxGetNumberOfFields, mxGetNzmax_730 as mxGetNzmax, mxGetPi, mxGetPr, mxGetPropertyShared,
        mxGetProperty_730 as mxGetProperty, mxGetScalar, mxGetString_730 as mxGetString,
        mxGetUserBits, mxIsCell, mxIsChar, mxIsClass, mxIsComplex, mxIsDouble, mxIsEmpty,
        mxIsFinite, mxIsFromGlobalWS, mxIsFunctionHandle, mxIsInf, mxIsInt16, mxIsInt32, mxIsInt64,
        mxIsInt8, mxIsLogical, mxIsLogicalScalar, mxIsLogicalScalarTrue, mxIsNaN, mxIsNumeric,
        mxIsObject, mxIsOpaque, mxIsScalar, mxIsSingle, mxIsSparse, mxIsStruct, mxIsUint16,
        mxIsUint32, mxIsUint64, mxIsUint8, mxLogical, mxMAXNAM, mxMalloc, mxRealloc, mxRemoveField,
        mxSetCell_730 as mxSetCell, mxSetClassName, mxSetData,
        mxSetDimensions_730 as mxSetDimensions, mxSetFieldByNumber_730 as mxSetFieldByNumber,
        mxSetField_730 as mxSetField, mxSetFromGlobalWS, mxSetImagData, mxSetIr_730 as mxSetIr,
        mxSetJc_730 as mxSetJc, mxSetM_730 as mxSetM, mxSetN_730 as mxSetN,
        mxSetNzmax_730 as mxSetNzmax, mxSetPi, mxSetPr, mxSetPropertyShared,
        mxSetProperty_730 as mxSetProperty, mxSetUserBits, mxUnreference, mxUnshareArray, Engine,
        MATFile, FILE, MWINDEX_MAX, MWINDEX_MIN, MWSINDEX_MAX, MWSINDEX_MIN, MWSIZE_MAX,
        MWSIZE_MIN, MW_FIRST_API_VERSION, MW_LATEST_API_VERSION,
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
        engOutputBuffer, engPutVariable, engSetVisible, fn_clean_up_after_error,
        fn_mex_get_local_function_table, fn_mex_set_local_function_table,
        fn_simple_function_to_string, matClose_800 as matClose,
        matDeleteVariable_800 as matDeleteVariable, matError, matGetDir_800 as matGetDir,
        matGetErrno_800 as matGetErrno, matGetFp_800 as matGetFp,
        matGetNextVariableInfo_800 as matGetNextVariableInfo,
        matGetNextVariable_800 as matGetNextVariable, matGetVariableInfo_800 as matGetVariableInfo,
        matGetVariable_800 as matGetVariable, matOpen_800 as matOpen,
        matPutVariableAsGlobal_800 as matPutVariableAsGlobal, matPutVariable_800 as matPutVariable,
        mexAtExit_800 as mexAtExit, mexCallMATLABWithTrap_800 as mexCallMATLABWithTrap,
        mexCallMATLAB_800 as mexCallMATLAB, mexErrMsgIdAndTxt_800 as mexErrMsgIdAndTxt,
        mexErrMsgTxt_800 as mexErrMsgTxt, mexEvalStringWithTrap_800 as mexEvalStringWithTrap,
        mexEvalString_800 as mexEvalString, mexFunction, mexFunctionName_800 as mexFunctionName,
        mexFunctionTable, mexFunctionTableEntry, mexGetVariablePtr_800 as mexGetVariablePtr,
        mexGetVariable_800 as mexGetVariable, mexGlobalTable, mexGlobalTableEntry,
        mexInitTermTableEntry, mexIsGlobal_800 as mexIsGlobal, mexIsLocked_800 as mexIsLocked,
        mexLocalFunctionTable, mexLock_800 as mexLock,
        mexMakeArrayPersistent_800 as mexMakeArrayPersistent,
        mexMakeMemoryPersistent_800 as mexMakeMemoryPersistent,
        mexPrintAssertion_800 as mexPrintAssertion, mexPrintf_800 as mexPrintf,
        mexPutVariable_800 as mexPutVariable, mexUnlock_800 as mexUnlock,
        mexWarnMsgIdAndTxt_800 as mexWarnMsgIdAndTxt, mexWarnMsgTxt_800 as mexWarnMsgTxt,
        mex_exit_fn, mwIndex, mwSignedIndex, mwSize, mxAddField_800 as mxAddField, mxArray,
        mxArrayToString_800 as mxArrayToString, mxArrayToUTF8String_800 as mxArrayToUTF8String,
        mxCalcSingleSubscript_800 as mxCalcSingleSubscript, mxCalloc_800 as mxCalloc, mxChar,
        mxClassID, mxClassID_mxCELL_CLASS, mxClassID_mxCHAR_CLASS, mxClassID_mxDOUBLE_CLASS,
        mxClassID_mxFUNCTION_CLASS, mxClassID_mxINDEX_CLASS, mxClassID_mxINT16_CLASS,
        mxClassID_mxINT32_CLASS, mxClassID_mxINT64_CLASS, mxClassID_mxINT8_CLASS,
        mxClassID_mxLOGICAL_CLASS, mxClassID_mxOBJECT_CLASS, mxClassID_mxOPAQUE_CLASS,
        mxClassID_mxSINGLE_CLASS, mxClassID_mxSTRUCT_CLASS, mxClassID_mxUINT16_CLASS,
        mxClassID_mxUINT32_CLASS, mxClassID_mxUINT64_CLASS, mxClassID_mxUINT8_CLASS,
        mxClassID_mxUNKNOWN_CLASS, mxClassID_mxVOID_CLASS, mxComplexDouble, mxComplexInt16,
        mxComplexInt32, mxComplexInt64, mxComplexInt8, mxComplexSingle, mxComplexUint16,
        mxComplexUint32, mxComplexUint64, mxComplexUint8, mxComplexity, mxComplexity_mxCOMPLEX,
        mxComplexity_mxREAL, mxCreateCellArray_800 as mxCreateCellArray,
        mxCreateCellMatrix_800 as mxCreateCellMatrix, mxCreateCharArray_800 as mxCreateCharArray,
        mxCreateCharMatrixFromStrings_800 as mxCreateCharMatrixFromStrings,
        mxCreateDoubleMatrix_800 as mxCreateDoubleMatrix,
        mxCreateDoubleScalar_800 as mxCreateDoubleScalar,
        mxCreateLogicalArray_800 as mxCreateLogicalArray,
        mxCreateLogicalMatrix_800 as mxCreateLogicalMatrix,
        mxCreateLogicalScalar_800 as mxCreateLogicalScalar,
        mxCreateNumericArray_800 as mxCreateNumericArray,
        mxCreateNumericMatrix_800 as mxCreateNumericMatrix,
        mxCreateSparseLogicalMatrix_800 as mxCreateSparseLogicalMatrix,
        mxCreateSparse_800 as mxCreateSparse,
        mxCreateStringFromNChars_800 as mxCreateStringFromNChars,
        mxCreateString_800 as mxCreateString, mxCreateStructArray_800 as mxCreateStructArray,
        mxCreateStructMatrix_800 as mxCreateStructMatrix,
        mxCreateUninitNumericArray_800 as mxCreateUninitNumericArray,
        mxCreateUninitNumericMatrix_800 as mxCreateUninitNumericMatrix,
        mxDestroyArray_800 as mxDestroyArray, mxDuplicateArray_800 as mxDuplicateArray,
        mxFree_800 as mxFree, mxFunctionPtr, mxGetCell_800 as mxGetCell,
        mxGetChars_800 as mxGetChars, mxGetClassID_800 as mxGetClassID,
        mxGetClassName_800 as mxGetClassName, mxGetComplexDoubles_800 as mxGetComplexDoubles,
        mxGetComplexInt16s_800 as mxGetComplexInt16s, mxGetComplexInt32s_800 as mxGetComplexInt32s,
        mxGetComplexInt64s_800 as mxGetComplexInt64s, mxGetComplexInt8s_800 as mxGetComplexInt8s,
        mxGetComplexSingles_800 as mxGetComplexSingles,
        mxGetComplexUint16s_800 as mxGetComplexUint16s,
        mxGetComplexUint32s_800 as mxGetComplexUint32s,
        mxGetComplexUint64s_800 as mxGetComplexUint64s,
        mxGetComplexUint8s_800 as mxGetComplexUint8s, mxGetData_800 as mxGetData,
        mxGetDimensions_800 as mxGetDimensions, mxGetDoubles_800 as mxGetDoubles,
        mxGetElementSize_800 as mxGetElementSize, mxGetEps_800 as mxGetEps,
        mxGetFieldByNumber_800 as mxGetFieldByNumber,
        mxGetFieldNameByNumber_800 as mxGetFieldNameByNumber,
        mxGetFieldNumber_800 as mxGetFieldNumber, mxGetField_800 as mxGetField,
        mxGetInf_800 as mxGetInf, mxGetInt16s_800 as mxGetInt16s, mxGetInt32s_800 as mxGetInt32s,
        mxGetInt64s_800 as mxGetInt64s, mxGetInt8s_800 as mxGetInt8s, mxGetIr_800 as mxGetIr,
        mxGetJc_800 as mxGetJc, mxGetLogicals_800 as mxGetLogicals, mxGetM_800 as mxGetM,
        mxGetNChars_800 as mxGetNChars, mxGetN_800 as mxGetN, mxGetNaN_800 as mxGetNaN,
        mxGetNumberOfDimensions_800 as mxGetNumberOfDimensions,
        mxGetNumberOfElements_800 as mxGetNumberOfElements,
        mxGetNumberOfFields_800 as mxGetNumberOfFields, mxGetNzmax_800 as mxGetNzmax,
        mxGetPr_800 as mxGetPr, mxGetProperty_800 as mxGetProperty, mxGetScalar_800 as mxGetScalar,
        mxGetSingles_800 as mxGetSingles, mxGetString_800 as mxGetString,
        mxGetUint16s_800 as mxGetUint16s, mxGetUint32s_800 as mxGetUint32s,
        mxGetUint64s_800 as mxGetUint64s, mxGetUint8s_800 as mxGetUint8s,
        mxGetUserBits_800 as mxGetUserBits, mxIsCell_800 as mxIsCell, mxIsChar_800 as mxIsChar,
        mxIsClass_800 as mxIsClass, mxIsComplex_800 as mxIsComplex, mxIsDouble_800 as mxIsDouble,
        mxIsEmpty_800 as mxIsEmpty, mxIsFinite_800 as mxIsFinite,
        mxIsFromGlobalWS_800 as mxIsFromGlobalWS, mxIsFunctionHandle_800 as mxIsFunctionHandle,
        mxIsInf_800 as mxIsInf, mxIsInt16_800 as mxIsInt16, mxIsInt32_800 as mxIsInt32,
        mxIsInt64_800 as mxIsInt64, mxIsInt8_800 as mxIsInt8,
        mxIsLogicalScalarTrue_800 as mxIsLogicalScalarTrue,
        mxIsLogicalScalar_800 as mxIsLogicalScalar, mxIsLogical_800 as mxIsLogical,
        mxIsNaN_800 as mxIsNaN, mxIsNumeric_800 as mxIsNumeric, mxIsObject_800 as mxIsObject,
        mxIsOpaque_800 as mxIsOpaque, mxIsScalar_800 as mxIsScalar, mxIsSingle_800 as mxIsSingle,
        mxIsSparse_800 as mxIsSparse, mxIsStruct_800 as mxIsStruct, mxIsUint16_800 as mxIsUint16,
        mxIsUint32_800 as mxIsUint32, mxIsUint64_800 as mxIsUint64, mxIsUint8_800 as mxIsUint8,
        mxLogical, mxMAXNAM, mxMakeArrayComplex_800 as mxMakeArrayComplex,
        mxMakeArrayReal_800 as mxMakeArrayReal, mxMalloc_800 as mxMalloc,
        mxRealloc_800 as mxRealloc, mxRemoveField_800 as mxRemoveField, mxSetCell_800 as mxSetCell,
        mxSetClassName_800 as mxSetClassName, mxSetComplexDoubles_800 as mxSetComplexDoubles,
        mxSetComplexInt16s_800 as mxSetComplexInt16s, mxSetComplexInt32s_800 as mxSetComplexInt32s,
        mxSetComplexInt64s_800 as mxSetComplexInt64s, mxSetComplexInt8s_800 as mxSetComplexInt8s,
        mxSetComplexSingles_800 as mxSetComplexSingles,
        mxSetComplexUint16s_800 as mxSetComplexUint16s,
        mxSetComplexUint32s_800 as mxSetComplexUint32s,
        mxSetComplexUint64s_800 as mxSetComplexUint64s,
        mxSetComplexUint8s_800 as mxSetComplexUint8s, mxSetData_800 as mxSetData,
        mxSetDimensions_800 as mxSetDimensions, mxSetDoubles_800 as mxSetDoubles,
        mxSetFieldByNumber_800 as mxSetFieldByNumber, mxSetField_800 as mxSetField,
        mxSetFromGlobalWS_800 as mxSetFromGlobalWS, mxSetInt16s_800 as mxSetInt16s,
        mxSetInt32s_800 as mxSetInt32s, mxSetInt64s_800 as mxSetInt64s,
        mxSetInt8s_800 as mxSetInt8s, mxSetIr_800 as mxSetIr, mxSetJc_800 as mxSetJc,
        mxSetM_800 as mxSetM, mxSetN_800 as mxSetN, mxSetNzmax_800 as mxSetNzmax,
        mxSetPr_800 as mxSetPr, mxSetProperty_800 as mxSetProperty,
        mxSetSingles_800 as mxSetSingles, mxSetUint16s_800 as mxSetUint16s,
        mxSetUint32s_800 as mxSetUint32s, mxSetUint64s_800 as mxSetUint64s,
        mxSetUint8s_800 as mxSetUint8s, mxSetUserBits_800 as mxSetUserBits, Engine, MATFile, FILE,
        MWINDEX_MAX, MWINDEX_MIN, MWSINDEX_MAX, MWSINDEX_MIN, MWSIZE_MAX, MWSIZE_MIN,
        MW_FIRST_API_VERSION, MW_LATEST_API_VERSION,
    };
}
