# Welcome to hip-validator 👋
![Version](https://img.shields.io/badge/version-1.0.0--alpha-blue.svg?cacheSeconds=2592000)

> Utility tool to validate the format of an HIP

### 🏠 [Homepage](https://holium.org/)

## Validation rules

### Mandatory fields

- `hip`: HIP number
- `title`: The HIP title is a few words, not a complete sentence
- `description`: Description is one full (short) sentence
- `author`: A list of the author's or authors' name(s) and/or username(s), or name(s) and email(s), e.g. (use with the 
parentheses or triangular brackets): FirstName LastName (@GitHubUsername), FirstName LastName <foo@bar.com>, FirstName
(@GitHubUsername) and GitHubUsername (@GitHubUsername)>
- `status`: HIP status
- `created`: Date created on, in ISO 8601 (yyyy-mm-dd) format

## Optional fields

- `requires`: HIP number(s)
- `replaces`: HIP number(s)

## Usage

```sh
cargo run --help
```

## Run tests

```sh
cargo test
```

## Author

👤 **Polyphene**

* Website: https://holium.org/
* Github: [@polyphene](https://github.com/polyphene)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/polyphene/hip-validator/issues). 

## Show your support

Give a ⭐️ if this project helped you!


***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_