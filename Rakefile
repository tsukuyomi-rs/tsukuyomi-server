require 'rake'

task :test do
    sh "if cargo clippy --version; then cargo clippy --all-targets; fi"
    sh "cargo test"
    sh "cargo test --all-features"
    sh "cargo test --no-default-features"
    sh "cargo test -p doctest"
end

task pre_release: [:test] do
    sh "cargo publish --dry-run"
end

task :install_hooks do
    sh "cargo clean -p cargo-husky"
    sh "cargo check -p cargo-husky"
end

task default: :test
