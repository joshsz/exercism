-module(hello_world_tests).

% This file is automatically generated from the exercises canonical data.

-define(TESTED_MODULE, (sut(hello_world))).
-define(TEST_VERSION, 2).
-include("exercism.hrl").


say_hi_test() ->
    ?assertEqual("Hello, World!", ?TESTED_MODULE:hello()).
