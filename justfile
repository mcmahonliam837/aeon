build:
    cargo build --workspace

test:
    cargo insta test --workspace

review:
  cargo insta review --workspace
