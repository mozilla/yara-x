# amo

This module contains helpers for AMO.

## Configuration

This module requires (strictly-valid) JSON metadata to be fully operational:

```json
{
  "manifest": { /* the content of the manifest.json file */ }
}
```

## Functions

### `manifest.get_value(string)`

This helper function allows to write conditions based on properties/values in
the `manifest.json`. It takes a [JSON path][] as input and returns a _string_
value (even for numbers and arrays, see examples below).

**Note:** If the (JSON) path is invalid, or if the property is not found, the
function returns an empty string.

#### Example

```yara
import "amo"

rule match_manifest_version {
  condition:
    // Numbers are represented as strings.
    amo.manifest.get_value("$.manifest_version") == "2"
}

rule match_extension_id {
  condition:
    amo.manifest.get_value("$.browser_specific_settings.gecko.id") == "some@id"
}

rule match_a_permission {
  condition:
    // Arrays are represented as a full JSON string, e.g. `["foo","bar","baz"]`.
    //
    // This is why it's possible to target a specific permission as follows:
    amo.manifest.get_value("$.permissions") contains "\"scripting\""
}
```

## Development

### Running tests

```
cargo test modules::amo::test
```

[json path]: https://datatracker.ietf.org/doc/html/rfc9535
