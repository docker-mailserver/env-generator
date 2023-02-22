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
    cargo run --quiet -- "${YAML_FILE}" "${DIRECTORY}/${BASENAME}.md" "${DIRECTORY}/${BASENAME}.env"
  else
    # echo "'${YAML_FILE}' is not a YAML file" >&2
    continue
  fi
done

MD_FINAL="${DIRECTORY}/combined.md.unfinished"
ENV_FINAL="${DIRECTORY}/combined.env.unfinished"

rm -f "${MD_FINAL%.unfinished}" "${ENV_FINAL%.unfinished}"
touch "${MD_FINAL}" "${ENV_FINAL}"

cat >"${MD_FINAL}" << EOM
---
title: Environment Variables
---

!!! attention

    If an option doesn't work as documented here, check if the version of the documentation fits the image version of DMS you are running. See [our tagging convention](../usage/#available-images-tags-tagging-convention).
EOM

cat >"${ENV_FINAL}" << EOM
# DOCUMENTATION FOR THESE VARIABLES IS FOUND UNDER
# https://docker-mailserver.github.io/docker-mailserver/edge/config/environment/

EOM

for MD_FILE in "${DIRECTORY}/"*.md
do
  SECTION_NAME=$(cut -d '-' -f 2 <<< "${MD_FILE%.md}")
  echo -e "\n## ${SECTION_NAME^}" >>"${MD_FINAL}"
  cat "${MD_FILE}" >>"${MD_FINAL}"
done

for ENV_FILE in "${DIRECTORY}/"*.env
do
  SECTION_NAME=$(cut -d '-' -f 2 <<< "${ENV_FILE%.env}")
  echo -e "# Section: ${SECTION_NAME^}\n" >>"${ENV_FINAL}"
  cat "${ENV_FILE}" >>"${ENV_FINAL}"
done

mv "${MD_FINAL}" "${MD_FINAL%.unfinished}"
mv "${ENV_FINAL}" "${ENV_FINAL%.unfinished}"
