#!/usr/bin/env bash
#MISE wait_for=["fix:name"]

set -euo pipefail

assert_toml_value() {
  local file=${1:?} key=${2:?} expected=${3?} actual
  actual=$(taplo get --file-path "$file" --strip-newline "$key") || {
    echo "failed to read $key from $file" >&2
    return 1
  }
  if [[ $actual != "$expected" ]]; then
    echo "expected $key to be \"$expected\", got \"$actual\"" >&2
    return 1
  fi
}

validate_fnox_config() {
  local config_json=${1:?}
  if ! jq --exit-status '
    .providers.age as $age
    | if (
        ($age | type) == "object"
        and $age.type == "age"
        and ($age.recipients | type) == "array"
      )
      then
        ($age.recipients | length) >= 1
        and all(
          $age.recipients[];
          if type == "string" then test("^age1[0-9a-z]+$") else false end
        )
        and (($age.recipients | unique | length) == ($age.recipients | length))
      else false
      end
  ' >/dev/null <<<"$config_json"; then
    echo "providers.age must have type = \"age\" and a non-empty array of unique age recipients" >&2
    return 1
  fi
}

fnox_toml=
while [[ $# -gt 0 ]]; do
  case "$1" in
  --fnox-toml)
    fnox_toml=${2:?"--fnox-toml requires a path"}
    shift 2
    ;;
  *)
    echo "usage: $0 [--fnox-toml <path>]" >&2
    exit 1
    ;;
  esac
done

project_root=${MISE_PROJECT_ROOT:-$(pwd)}
fnox_toml=${fnox_toml:-"$project_root/fnox.toml"}
project_name=$(mise --cd "$project_root" run git:repo-name)
assert_toml_value "$fnox_toml" "providers.keychain.service" "$project_name"
assert_toml_value "$fnox_toml" "providers.pass.prefix" "$project_name/"
config_json=$(taplo get --file-path "$fnox_toml" --output-format json)
validate_fnox_config "$config_json"
