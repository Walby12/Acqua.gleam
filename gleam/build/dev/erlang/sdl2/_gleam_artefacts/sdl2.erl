-module(sdl2).
-compile([no_auto_import, nowarn_unused_vars, nowarn_unused_function, nowarn_nomatch]).

-export([command_to_json/0, main/0]).

-file("src/sdl2.gleam", 4).
-spec command_to_json() -> binary().
command_to_json() ->
    _pipe = gleam@json:object(
        [{<<"func"/utf8>>, gleam@json:string(<<"fill to ppm"/utf8>>)},
            {<<"pixels"/utf8>>,
                gleam@json:array([800 * 600], fun gleam@json:int/1)},
            {<<"width"/utf8>>, gleam@json:int(800)},
            {<<"height"/utf8>>, gleam@json:int(600)},
            {<<"color"/utf8>>, gleam@json:int(16#FF00FF00)},
            {<<"file path"/utf8>>, gleam@json:string(<<"../out.ppm"/utf8>>)}]
    ),
    gleam@json:to_string(_pipe).

-file("src/sdl2.gleam", 16).
-spec main() -> {ok, nil} | {error, simplifile:file_error()}.
main() ->
    Content = command_to_json(),
    Match = begin
        _pipe = Content,
        simplifile:write(<<"../main.json"/utf8>>, _pipe)
    end,
    case Match of
        {ok, _} ->
            Match;

        {error, _} ->
            erlang:error(#{gleam_error => panic,
                    message => <<"ERROR: error during metadate as json"/utf8>>,
                    module => <<"sdl2"/utf8>>,
                    function => <<"main"/utf8>>,
                    line => 23})
    end.
