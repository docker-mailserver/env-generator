#! /bin/bash

set -eE -u -o pipefail
shopt -s globstar inherit_errexit

DIRECTORY=${1:?Directory that contains YAML files must be specified}

if [[ ! -d ${DIRECTORY} ]]
then
  echo "Directory '${DIRECTORY}' does not exist" >&2
  exit 1
fi

for YAML_FILE in "${DIRECTORY}/"*
do
  if [[ ${YAML_FILE} =~ .+\.y(a)?ml$ ]]
  then
    BASENAME=$(basename "${YAML_FILE}")
    BASENAME=${BASENAME%.yaml} ; BASENAME=${BASENAME%.yml} ; BASENAME=${BASENAME,,} ;
    cargo run -- "${YAML_FILE}" "${DIRECTORY}/${BASENAME}.md" "${DIRECTORY}/${BASENAME}.env"
  else
    # echo "'${YAML_FILE}' is not a YAML file" >&2
    continue
  fi
done

# TODO combine the previously created files into 1
