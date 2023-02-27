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

%% Test explore
% Generate a log file from running explore with known inputs and compare it with a log file generated from the 
% original C-example and the same inputs.
int8_5d_array = int8(ones(3,3,3,3,3));
uint8_5d_array = uint8(ones(3,3,3,3,3));
int16_5d_array = int16(ones(3,3,3,3,3));
uint16_5d_array = uint16(ones(3,3,3,3,3));
int32_5d_array = int32(ones(3,3,3,3,3));
uint32_5d_array = uint32(ones(3,3,3,3,3));
int64_5d_array = int64(ones(3,3,3,3,3));
uint64_5d_array = uint64(ones(3,3,3,3,3));
single_5d_array = single(ones(3,3,3,3,3));
double_5d_array = double(ones(3,3,3,3,3));
struct_5d_array = repmat(struct('a',1,'b',"string",'c',struct('inner',"inner_struct")),[3,3,3,3,3]);
cell_5d_array = cell(3,3,3,3,3); cell_5d_array{1,1,1,1,1} = int8_5d_array; cell_5d_array{1,1,1,1,2} = struct_5d_array;
string_5d_array = repmat("I am a string",[3,3,3,3,3]);
diary test_explore_actual_temp.txt
explore(int8_5d_array);
explore(uint8_5d_array);
explore(int16_5d_array);
explore(uint16_5d_array);
explore(int32_5d_array);
explore(uint32_5d_array);
explore(int64_5d_array);
explore(uint64_5d_array);
explore(single_5d_array);
explore(double_5d_array);
explore(struct_5d_array);
explore(cell_5d_array);
explore(string_5d_array);
diary off

test_explore_actual = fileread("test_explore_actual_temp.txt");
delete test_explore_actual_temp.txt
test_explore_reference = fileread("test_explore_reference.txt");
% Replace all carriage return characters in both strings because of git-windows interaction
test_explore_actual(test_explore_actual == char(13)) == '';
test_explore_reference(test_explore_reference == char(13)) = '';
assert(all(test_explore_reference == test_explore_actual));