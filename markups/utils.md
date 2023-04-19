# Utils

This contains Rust library crates containing two sub-modules - 
1. auth 
2. security 

They are designed to handle authentication and security management in the application.

## Auth Module

The auth module provides functions for generating and verifying JSON Web Tokens (JWTs) using the jsonwebtoken-rs crate. With this module, you can easily create JWTs to authenticate users and verify them to ensure that requests are coming from authenticated sources.

To use the auth module in your Rust project, simply add the following line to your Cargo.toml file:

```
[dependencies]
jsonwebtoken = "8.2.1"
```


You can then import the auth module in your Rust code using:

```
use my_cool_app::auth;
```

## Security Module

The security module contains Rust code to handle environment and security management at the application level. This module provides functions to handle things like generating secure random passwords, encrypting and decrypting data, and managing environment variables.

To use the security module in your Rust project, add the following line to your Cargo.toml file:

```
[dependencies]
rand = "0.8.4"
```

You can then import the security module in your Rust code using:

```
use my_cool_app::security;
```

## Cargo Workspace

This Rust library is organized as a Cargo workspace, with the auth and security sub-modules contained within the same project. This makes it easy to manage and build the entire library as a single unit.

To use this Rust library in your project, simply add the following to your Cargo.toml file:

```
[dependencies]
my_cool_app = { git = "https://github.com/myusername/my_cool_app.git", branch = "main" }
```

You can then import the auth and security modules in your Rust code as shown above.

## Conclusion

That's it! With this Rust library, you can easily handle authentication and security management in your application. Happy coding!