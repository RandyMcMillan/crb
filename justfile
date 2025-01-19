version := `git tag -l --sort -version:refname | head -n 1 | sed -e 's/v//g'`
tag := "v" + version

bump:
    cargo set-version --workspace --bump patch

tag:
    git tag {{tag}}
    git push origin {{tag}}

release:
    cargo workspaces publish --allow-branch trunk --all --publish-as-is
