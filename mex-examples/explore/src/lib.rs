#![allow(non_upper_case_globals)]

use std::ops::Rem;
use matlab_sys::interleaved_complex as raw;

/* Pass analyze_cell a pointer to a cell mxArray.  Each element
in a cell mxArray is called a "cell"; each cell holds zero
or one mxArray.  analyze_cell accesses each cell and displays
information about it. */
unsafe fn analyze_cell(cell_array_pointer: *const raw::mxArray) {
    let total_num_of_cells = raw::mxGetNumberOfElements(cell_array_pointer);
    raw::mexPrintf(
        b"total num of cells = %d\n\0".as_ptr().cast(),
        total_num_of_cells,
    );
    raw::mexPrintf(b"\n\0".as_ptr().cast());
    /* Each cell mxArray contains m-by-n cells; Each of these cells
    is an mxArray. */
    for index in 0..total_num_of_cells {
        raw::mexPrintf(b"\n\n\t\tCell Element: \0".as_ptr().cast());
        display_subscript(cell_array_pointer, index);
        raw::mexPrintf(b"\n\0".as_ptr().cast());
        let cell_element_ptr = raw::mxGetCell(cell_array_pointer, index);
        if cell_element_ptr.is_null() {
            raw::mexPrintf(b"\tEmpty Cell\n\0".as_ptr().cast());
        } else {
            /* Display a top banner. */
            raw::mexPrintf(
                b"------------------------------------------------\n\0"
                    .as_ptr()
                    .cast(),
            );
            get_characteristics(cell_element_ptr);
            analyze_class(cell_element_ptr);
            raw::mexPrintf(b"\n\0".as_ptr().cast());
        }
    }
    raw::mexPrintf(b"\n\0".as_ptr().cast());
}

/* Pass analyze_structure a pointer to a structure mxArray.  Each element
in a structure mxArray holds one or more fields; each field holds zero
or one mxArray.  analyze_structure accesses every field of every
element and displays information about it. */
unsafe fn analyze_structure(structure_array_ptr: *const raw::mxArray) {
    raw::mexPrintf(b"\n\0".as_ptr().cast());
    let total_num_of_elements = raw::mxGetNumberOfElements(structure_array_ptr);
    let number_of_fields = raw::mxGetNumberOfFields(structure_array_ptr);

    /* Walk through each structure element. */
    for index in 0..total_num_of_elements {
        /* For the given index, walk through each field. */
        for field_index in 0..number_of_fields {
            raw::mexPrintf(b"\n\t\t\0".as_ptr().cast());
            display_subscript(structure_array_ptr, index);
            let field_name = raw::mxGetFieldNameByNumber(structure_array_ptr, field_index);
            raw::mexPrintf(b".%s\n\0".as_ptr().cast(), field_name);
            let field_array_ptr = raw::mxGetFieldByNumber(structure_array_ptr, index, field_index);
            if field_array_ptr.is_null() {
                raw::mexPrintf(b"\tEmpty Field\n\0".as_ptr().cast());
            } else {
                /* Display a top banner. */
                raw::mexPrintf(
                    b"------------------------------------------------\n\0"
                        .as_ptr()
                        .cast(),
                );
                get_characteristics(field_array_ptr);
                analyze_class(field_array_ptr);
                raw::mexPrintf(b"\n\0".as_ptr().cast());
            }
        }
        raw::mexPrintf(b"\n\n\0".as_ptr().cast());
    }
}

/* Pass analyze_string a pointer to a char mxArray.  Each element
in a char mxArray holds one 2-byte character (an mxChar);
analyze_string displays the contents of the input char mxArray
one row at a time.  Since adjoining row elements are NOT stored in
successive indices, analyze_string has to do a bit of math to
figure out where the next letter in a string is stored. */
unsafe fn analyze_string(string_array_ptr: *const raw::mxArray) {
    /* Allocate enough memory to hold the converted string. */
    let buflen = raw::mxGetNumberOfElements(string_array_ptr) + 1;
    let buf = raw::mxCalloc(buflen, std::mem::size_of::<core::ffi::c_char>()).cast();

    /* Copy the string data from string_array_ptr and place it into buf. */
    if raw::mxGetString(string_array_ptr, buf, buflen) != 0 {
        raw::mexErrMsgIdAndTxt(
            b"MATLAB:explore:invalidStringArray\0".as_ptr().cast(),
            b"Could not convert string data.\0".as_ptr().cast(),
        );
    }

    /* Get the shape of the input mxArray. */
    let dims = raw::mxGetDimensions(string_array_ptr);
    let number_of_dimensions = raw::mxGetNumberOfDimensions(string_array_ptr);

    let elements_per_page = *dims.add(0) * *dims.add(1);
    /* total_number_of_pages = dims[2] x dims[3] x ... x dims[N-1] */
    let mut total_number_of_pages = 1;
    for d in 2..number_of_dimensions {
        total_number_of_pages *= *dims.add(d);
    }

    for page in 0..total_number_of_pages {
        /* On each page, walk through each row. */
        for row in 0..*dims.add(0) {
            let mut index = (page * elements_per_page) + row;
            raw::mexPrintf(b"\t\0".as_ptr().cast());
            display_subscript(string_array_ptr, index);
            raw::mexPrintf(b" ".as_ptr().cast());

            /* Walk along each column in the current row. */
            for _column in 0..*dims.add(1) {
                raw::mexPrintf(b"%c\0".as_ptr().cast(), *buf.add(index) as core::ffi::c_int);
                index += *dims.add(0);
            }
            raw::mexPrintf(b"\n".as_ptr().cast());
        }
    }
}

/* Pass analyze_sparse a pointer to a sparse mxArray.  A sparse mxArray
only stores its nonzero elements.  The values of the nonzero elements
are stored in the pr and pi arrays.  The tricky part of analyzing
sparse mxArray's is figuring out the indices where the nonzero
elements are stored.  (See the mxSetIr and mxSetJc reference pages
for details. */
unsafe fn analyze_sparse(array_ptr: *const raw::mxArray) {
    let mut total = 0;
    // Using the interleaved-complex API
    if raw::mxIsComplex(array_ptr) {
        let pc = raw::mxGetComplexDoubles(array_ptr);
        let ir = raw::mxGetIr(array_ptr);
        let jc = raw::mxGetJc(array_ptr);
        /* Display the nonzero elements of the sparse array. */
        let n = raw::mxGetN(array_ptr);
        for col in 0..n {
            let starting_row_index = *jc.add(col);
            let stopping_row_index = *jc.add(col + 1);
            if starting_row_index == stopping_row_index {
                continue;
            } else {
                for current_row_index in starting_row_index..stopping_row_index {
                    raw::mexPrintf(
                        "\t(%zu,%zu) = %g+%g i\n".as_ptr().cast(),
                        *ir.add(current_row_index) + 1,
                        col + 1,
                        (*pc.add(total)).real,
                        (*pc.add(total)).imag,
                    );
                    total += 1;
                }
            }
        }
    } else {
        let p = raw::mxGetDoubles(array_ptr);
        let ir = raw::mxGetIr(array_ptr);
        let jc = raw::mxGetJc(array_ptr);
        let n = raw::mxGetN(array_ptr);
        for col in 0..n {
            let starting_row_index = *jc.add(col);
            let stopping_row_index = *jc.add(col + 1);
            if starting_row_index == stopping_row_index {
                continue;
            } else {
                for current_row_index in starting_row_index..stopping_row_index {
                    raw::mexPrintf(
                        "\t(%zu,%zu) = %g\n".as_ptr().cast(),
                        *ir.add(current_row_index) + 1,
                        col + 1,
                        *p.add(total),
                    );
                    total += 1;
                }
            }
        }
    }
}

macro_rules! analyze_num {
    ($fn_name:ident, $get_complex:ident, $get_real:ident, $complex_fmt_str:expr, $real_fmt_str:expr, $promoted_type:ty) => {
        unsafe fn $fn_name(array_ptr: *const raw::mxArray) {
            // Use interleaved-complex API
            let total_num_of_elements = raw::mxGetNumberOfElements(array_ptr);
            if raw::mxIsComplex(array_ptr) {
                let mut pc = raw::$get_complex(array_ptr);
                for index in 0..total_num_of_elements {
                    raw::mexPrintf(b"\t\0".as_ptr().cast());
                    display_subscript(array_ptr, index);
                    raw::mexPrintf(
                        $complex_fmt_str.as_ptr().cast(),
                        (*pc).real as $promoted_type,
                        (*pc).imag as $promoted_type,
                    );
                    pc = pc.add(1);
                }
            } else {
                let mut p = raw::$get_real(array_ptr);
                for index in 0..total_num_of_elements {
                    raw::mexPrintf(b"\t\0".as_ptr().cast());
                    display_subscript(array_ptr, index);
                    raw::mexPrintf($real_fmt_str.as_ptr().cast(), *p as $promoted_type);
                    p = p.add(1);
                }
            }
        }
    };
}

analyze_num!(
    analyze_int8,
    mxGetComplexInt8s,
    mxGetInt8s,
    b" = %d + %di\n\0",
    b" = %d\n\0",
    core::ffi::c_int
);

analyze_num!(
    analyze_uint8,
    mxGetComplexUint8s,
    mxGetUint8s,
    b" = %u + ui\n\0",
    b" = %d\n\0",
    core::ffi::c_uint
);

analyze_num!(
    analyze_int16,
    mxGetComplexInt16s,
    mxGetInt16s,
    b" = %d + %di\n\0",
    b" = %d\n\0",
    core::ffi::c_int
);

analyze_num!(
    analyze_uint16,
    mxGetComplexUint16s,
    mxGetUint16s,
    b" = %u + %ui\n\0",
    b" = %u\n\0",
    core::ffi::c_uint
);

analyze_num!(
    analyze_int32,
    mxGetComplexInt32s,
    mxGetInt32s,
    b" = %d + %di\n\0",
    b" = %d\n\0",
    core::ffi::c_int
);

analyze_num!(
    analyze_uint32,
    mxGetComplexUint32s,
    mxGetUint32s,
    b" = %u + %ui\n\0",
    b" = %u\n\0",
    core::ffi::c_uint
);

analyze_num!(
    analyze_int64,
    mxGetComplexInt64s,
    mxGetInt64s,
    b" = %zd + %zdi\n\0",
    b" = %zd\n\0",
    i64
);

analyze_num!(
    analyze_uint64,
    mxGetComplexUint64s,
    mxGetUint64s,
    b" = %zu + %zui\n\0",
    b" = %zu\n\0",
    u64
);

analyze_num!(
    analyze_single,
    mxGetComplexSingles,
    mxGetSingles,
    b" = %g + %gi\n\0",
    b" = %g\n\0",
    f64
);

analyze_num!(
    analyze_double,
    mxGetComplexDoubles,
    mxGetDoubles,
    b" = %g + %gi\n\0",
    b" = %g\n\0",
    f64
);

unsafe fn analyze_logical(array_ptr: *const raw::mxArray) {
    let total_num_of_elements = raw::mxGetNumberOfElements(array_ptr);
    let pr = raw::mxGetData(array_ptr) as *mut raw::mxLogical;
    for index in 0..total_num_of_elements {
        raw::mexPrintf(b"\t\0".as_ptr().cast());
        display_subscript(array_ptr, index);
        if *pr {
            raw::mexPrintf(b" = true\n\0".as_ptr().cast());
        } else {
            raw::mexPrintf(b" = false\n\0".as_ptr().cast());
        }
    }
}

/* Pass analyze_full a pointer to any kind of numeric mxArray.
analyze_full figures out what kind of numeric mxArray this is. */
unsafe fn analyze_full(numeric_array_ptr: *const raw::mxArray) {
    use raw::mxClassID::*;
    let category = raw::mxGetClassID(numeric_array_ptr);
    match category {
        mxINT8_CLASS => analyze_int8(numeric_array_ptr),
        mxUINT8_CLASS => analyze_uint8(numeric_array_ptr),
        mxINT16_CLASS => analyze_int16(numeric_array_ptr),
        mxUINT16_CLASS => analyze_uint16(numeric_array_ptr),
        mxINT32_CLASS => analyze_int32(numeric_array_ptr),
        mxUINT32_CLASS => analyze_uint32(numeric_array_ptr),
        mxINT64_CLASS => analyze_int64(numeric_array_ptr),
        mxUINT64_CLASS => analyze_uint64(numeric_array_ptr),
        mxSINGLE_CLASS => analyze_single(numeric_array_ptr),
        mxDOUBLE_CLASS => analyze_double(numeric_array_ptr),
        _ => (),
    }
}

/* Display the subscript associated with the given index. */
unsafe fn display_subscript(array_ptr: *const raw::mxArray, index: raw::mwSize) {
    let number_of_dimensions = raw::mxGetNumberOfDimensions(array_ptr);
    let subscript: *mut raw::mwSize =
    raw::mxCalloc(number_of_dimensions, core::mem::size_of::<raw::mwSize>()).cast();
    let dims = raw::mxGetDimensions(array_ptr);
    raw::mexPrintf(b"(\0".as_ptr().cast());
    let mut subindex = index;
    for d in (0..number_of_dimensions).rev() {
        let mut total = 1;
        for inner in 0..d{
            total *= *dims.add(inner);
        }
        *subscript.add(d) = subindex / total;
        subindex = subindex.rem(total);
        if d == 0 {
            break;
        }
    }
    for q in 0..number_of_dimensions - 1 {
        raw::mexPrintf(b"%d,\0".as_ptr().cast(), *subscript.add(q) + 1);
    }
    raw::mexPrintf(
        b"%d)\0".as_ptr().cast(),
        *subscript.add(number_of_dimensions - 1) + 1,
    );
    raw::mxFree(subscript.cast());
}

/* get_characteristics figures out the size, and category
of the input array_ptr, and then displays all this information. */
unsafe fn get_characteristics(array_ptr: *const raw::mxArray) {
    use ::std::io::Write;
    /* Display the mxArray's Dimensions; for example, 5x7x3.
    If the mxArray's Dimensions are too long to fit, then just
    display the number of dimensions; for example, 12-D. */
    let number_of_dimensions = raw::mxGetNumberOfDimensions(array_ptr);
    let dims = raw::mxGetDimensions(array_ptr);

    /* alloc memory for shape_string w.r.t thrice the number of dimensions */
    /* (so that we can also add the 'x')                                   */
    let shape_string_len = number_of_dimensions * 3;
    let mut shape_cursor = std::io::Cursor::new(core::slice::from_raw_parts_mut(
        raw::mxCalloc(shape_string_len, core::mem::size_of::<u8>()).cast(),
        shape_string_len,
    ));

    for c in 0..number_of_dimensions {
        write!(shape_cursor, "{}x", *dims.add(c)).unwrap();
    }
    let length_of_shape_string = shape_cursor.position();
    /* replace the last 'x' with a space */
    shape_cursor.set_position(length_of_shape_string - 1);
    shape_cursor.write(&[b'\0']).unwrap();
    if length_of_shape_string > 16 {
        shape_cursor.set_position(0);
        write!(shape_cursor, "{}-D", number_of_dimensions).unwrap();
        shape_cursor.write(&[b'\0']).unwrap();
    }
    let shape_string: *mut u8 = shape_cursor.into_inner().as_mut_ptr();
    raw::mexPrintf(b"Dimensions: %s\n\0".as_ptr().cast(), shape_string);
    /* Display the mxArray's class (category). */
    let class_name = raw::mxGetClassName(array_ptr);
    raw::mexPrintf(
        b"Class Name: %s%s\n\0".as_ptr().cast(),
        class_name,
        if raw::mxIsSparse(array_ptr) {
            b" (sparse)\0".as_ptr().cast::<core::ffi::c_char>()
        } else {
            b"\0".as_ptr().cast::<core::ffi::c_char>()
        },
    );
    /* Display a bottom banner. */
    raw::mexPrintf(
        b"------------------------------------------------\n\0"
            .as_ptr()
            .cast(),
    );
    /* free up memory for shape_string */
    raw::mxFree(shape_string.cast());
}

/* Determine the category (class) of the input array_ptr, and then
branch to the appropriate analysis routine. */
unsafe fn analyze_class(array_ptr: *const raw::mxArray) -> raw::mxClassID::Type {
    use raw::mxClassID::*;
    let category = raw::mxGetClassID(array_ptr);

    if raw::mxIsSparse(array_ptr) {
        analyze_sparse(array_ptr);
    } else {
        match category {
            mxLOGICAL_CLASS => analyze_logical(array_ptr),
            mxCHAR_CLASS => analyze_string(array_ptr),
            mxSTRUCT_CLASS => analyze_structure(array_ptr),
            mxCELL_CLASS => analyze_cell(array_ptr),
            mxUNKNOWN_CLASS => raw::mexWarnMsgIdAndTxt(
                b"MATLAB:explore:unknownClass\0".as_ptr().cast(),
                b"Unknown class.\0".as_ptr().cast(),
            ),
            _ => analyze_full(array_ptr),
        }
    }
    category
}

/* mexFunction is the gateway routine for the MEX-file. */
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn mexFunction(
    _nlhs: i32,
    _plhs: *mut *mut raw::mxArray,
    nrhs: i32,
    prhs: *const *const raw::mxArray,
) {
    // No need to check for MX_COMPAT_32 compile flag, since the bindings are only provided for the 64-bit variant.

    /* Look at each input (right-hand-side) argument. */
    for i in 0i32..nrhs {
        raw::mexPrintf(b"\n\n\0".as_ptr().cast());
        /* Display a top banner. */
        raw::mexPrintf(
            b"------------------------------------------------\n\0"
                .as_ptr()
                .cast(),
        );
        /* Display which argument */
        raw::mexPrintf(
            b"Name: %s%d%c\n\0".as_ptr().cast(),
            b"prhs[\0".as_ptr().cast::<core::ffi::c_char>(),
            i,
            b']' as core::ffi::c_int,
        );
        get_characteristics(*prhs.add(i as usize));
        analyze_class(*prhs.add(i as usize));
    }
}
