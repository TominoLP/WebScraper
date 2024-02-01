# Rust Web Scraper

This is my first Rust project, a basic web scraper. It utilizes the reqwest library for handling web requests and select for HTML parsing. The scraper extracts links from a webpage, showcasing my initial exploration into Rust programming.

---

## Workflow:

### Branches
- Pushes to the "devel" branch trigger the workflow.
- Pull requests targeting the "devel" branch also trigger the workflow.

### Build Job
- The workflow runs on the latest Ubuntu environment.
- It checks out the code using GitHub Actions.
- The build step uses `cargo` to build the Rust project with verbose output.
