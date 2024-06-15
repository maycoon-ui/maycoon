# Contributing to Maycoon

First off, thanks for taking the time to contribute! We appreciate your support.

The following is a set of guidelines for contributing to Maycoon. These guidelines are intended to make it easy for you to get involved with us.

## Table of Contents

1. [How Can I Contribute?](#how-can-i-contribute)
    - [Reporting Bugs](#reporting-bugs)
    - [Suggesting Enhancements](#suggesting-enhancements)
    - [Submitting Pull Requests](#submitting-pull-requests)
2. [Code of Conduct](#code-of-conduct)
3. [Style Guides](#style-guides)
    - [Coding Standards](#coding-standards)
    - [Commit Messages](#commit-messages)
4. [Contact](#contact)

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please report it by opening an issue in the [issue tracker](https://github.com/maycoon-ui/maycoon/issues). Make sure to include:

- A clear and descriptive title.
- A detailed description of the steps to reproduce the issue.
- The expected and actual results.
- Any relevant logs, screenshots, or other context.

### Suggesting Enhancements

If you have an idea to improve Maycoon, we would love to hear about it! To suggest an enhancement:

- Open an issue in the [issue tracker](https://github.com/maycoon-ui/maycoon/issues) with the label "enhancement".
- Describe your idea and explain why it would be useful.

### Submitting Pull Requests

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Commit your changes.
5. Push to the branch.
6. Open a Pull Request.

Please ensure that your pull request adheres to the following guidelines:

- Include a clear description of the changes and why they are being made.
- Follow the project's coding standards.
- Ensure that your changes pass the existing tests and add new tests if necessary.

## Code of Conduct

We are committed to maintaining a welcoming and respectful community. By participating, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md).

## Style Guides

### Coding Standards

- Make sure to use `rustfmt`, `cargo fix` and `clippy` for formatting and fixing code.
- Your code must be 100% documented. Every crate should have `#![warn(missing_docs)]` in the root file to warn about missing documentation.
- Try to be safe and use only `unsafe` code if really necessary.
- Before adding any dependencies, make sure you really need the crate functionality and cannot implement features yourself.
- Keep your code and dependencies up to date.
- Comment complex code for future contributors.

### Commit Messages

- Only give essential information about the commit.
- Reference issues and pull requests related to the commit.
- Don't leave out important information.

## Contact

If you have any questions or need further assistance, please reach out to the Author Mikail Plotzky via mp@ypon.com.

Thank you for contributing to Maycoon!
