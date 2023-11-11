# Welcome to the Shamba Up 🌱 GraphQL services!

## Getting Started 🚀
### Prerequisites / Installations 👨🏽‍💻
- [Rust](https://www.rust-lang.org/tools/install)
- [SurrealDB](https://surrealdb.com/install)

### Running the services 🏃🏽‍♂️
- Clone the repo
- Start SurrealDB
- Run any service using the command: `cargo watch -x run --workdir services/<directory_of_the_service>` in the root directory. This will start the server and restart it whenever you make changes to the code.\
e.g. `cargo watch -x run --workdir services/acl-service` will start the ACL service.\
Cargo will automatically install any dependencies that are missing.

### Testing the services endpoints 🧪
We are using GraphQL for our API. You can use any GraphQL client to test the endpoints.\
By default, there is a GraphQL playground available at `http://localhost:<PORT>` for each service.\
e.g. `http://localhost:3001` for the ACL service.

## Contributing 🤝🏽
Every authorized contributor is allowed to contribute to the repository whether by adding features, bug fixing or participating in code reviews. All code reviews are done on GitHub by the Lead Engineer.\

### Branching 🌳
- The `main` branch is the default branch and is protected. No one is allowed to push directly to the `main` branch.

- The `dev` branch is the development branch. All work should be done on the `dev` branch. The `dev` branch is protected. No one is allowed to push directly to the `dev` branch.

- All work should be done on a separate branch. The branch name should be in the format: `feature/<feature_name>` or `bug/<bug_name>`

- When you are done with your work, create a pull request to the `dev` branch. The pull request should be in the format: `feature/<feature_name>` or `bug/<bug_name>`

- The Lead Engineer will review the pull request and merge it to the `dev` branch if it is approved.\
- The Lead Engineer will merge the `dev` branch to the `main` branch when a new release is ready.

## License 📝
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏🏽
- [SurrealDB](https://surrealdb.com)
- [Rust](https://www.rust-lang.org/)
- [GraphQL](https://graphql.org/)
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/introduction.html)
- [Axum](https://github.com/tokio-rs/axum)

## Authors ✍🏽
- [Elon Aseneka Idiong'o](https://github.com/elonaire)
- [Kelvin Mwenda Kaburu]()
- [Simeon Omwoyo Maranga]()

