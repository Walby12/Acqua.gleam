import gleam/json
import simplifile

pub fn command_to_json() -> String {
  json.object([
    #("func", json.string("fill to ppm")),
    #("pixels", json.array([800*600], of: json.int)),
    #("width", json.int(800)),
    #("height", json.int(600)),
    #("color", json.int(0xFF00FF00)),
    #("file path", json.string("../out.ppm")),
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
