# (For maintainers) How to cut a new version

This is a checklist (mostly for me) on how to cut a new version of MistQL.

1. Cut a new branch from `main` named `X.Y.Z`
1. Update the CHANGELOG
1. Update the version in `py/mistql/__init__.py`
1. Update the version in `py/pyproject.toml`
1. Update the version in `js/package.json`
1. Update the version in `meta.yaml`
1. One final test via `talc test all`
1. Commit the changes (You should have 5 files changed)
1. Publish the npm and pypi packages via `talc publish js` and `talc publish py`
1. Update the doc version of MistQL
1. Version the docs via `npm run docusaurus docs:version X.Y.Z`
1. Commit the changes
1. Push the branch
1. Create a PR and merge it
