pub const TEMPLATE: &str = r#"
# Your github OAuth token
#
# You can use OAuth tokens to interact with GitHub via automated scripts.
#   https://docs.github.com/en/free-pro-team@latest/github/extending-github/git-automation-with-oauth-tokens
# Get your personal access token.
#   https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token
#
# Use for fetching substrate updates
# Or create/upgrade issue for you substrate project (require write access if you want to use these features)
github-oauth-token = "oauth-token"

# Your substrate project information
#substrate-project:
#  # https://github.com/{owner}/{repo}
#  owner: "owner"
#  repo: "repo"
#  issue-repo: "issue-repo"
#  local-full-path: "/path/to/project"
#  runtimes:
#	- branch: main
#	  runtime-relative-path: "path/to/runtime/src/lib.rs"
#	  node-rpc-uri: "http://127.0.0.1:9933"
"#;
