version := `git tag -l --sort -version:refname | head -n 1 | sed -e 's/v//g'`
tag := "v" + version

install-all:axum-handler crates-dumper file-watcher tui-app
axum-handler:
    cargo install --path examples/axum-handler
crates-dumper:
    cargo install --path examples/crates-dumper
file-watcher:
    cargo install --path examples/file-watcher
tui-app:
    cargo install --path examples/tui-app

bump:
    cargo set-version --workspace --bump patch

tag:
    git tag {{tag}}
    git push origin {{tag}}

release:
    cargo workspaces publish --allow-branch trunk --all --publish-as-is
