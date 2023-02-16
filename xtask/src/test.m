% A matlab unit test script file. Infos at https://de.mathworks.com/help/matlab/matlab_prog/write-script-based-unit-tests.html#d124e81995.


%% Test array_product
assert(isequal(array_product(5,[2,3,5,7]), [10,15,25,35]));

%% Test mex_call_matlab
mex_call_matlab();
assert(true);

%% Test array_size
required_size = array_size(10);
expected_size = 100 / 1024; % In kibibyte
assert(required_size == expected_size);