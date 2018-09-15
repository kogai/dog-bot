#!/bin/sh -eu

BRANCH="deploy-$(date -I)"
if [ "$1" = "prepare" ]; then
  git checkout -b "${BRANCH}" heroku/master
  echo "Update conversion.json"
elif [ "$1" = "deploy" ]; then
  echo "Deploying..."
  git push -f heroku "${BRANCH}":master
fi

