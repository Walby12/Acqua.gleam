import gleam/json
import simplifile

pub fn command_to_json() -> String {
  json.object([
    #("name", json.string("draw_line")),
    #("x1", json.int(10)),
    #("y1", json.int(10)),
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
