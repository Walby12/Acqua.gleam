import gleam/json
import simplifile

pub fn command_to_json() -> String {
  json.object([
    #("func", json.string("init_window")),
    #("name", json.string("test")),
    #("color", json.array([100,100,100], of: json.int))
  ])
  |> json.to_string
}

pub fn main() {
  let content = command_to_json()

  let match = content |> simplifile.write(to: "../main.json")

  case match {
    Ok(_) -> match
    Error(_) -> panic as "ERROR: error during metadate as json"
  }
}
