## Tasks
### 0. Create project
```bash
cargo new loansasa
```
### 1. Add actix server
- Add `actix-web`(**AW**) dependency in `.toml` to handle web server
- In the main async fn in main.rs:
    * Import the `HTTPServer` from `AW` dependency
    * Import `Http server` from `AW` and start a new instance
    * Bind to local host port 8080, run it and await

## 2. Add register GET route
- In the `main.rs`:
    * Import the `web` & `App` mod from `AW`
    * Create a new `App` instance in the `HTTPServer`
    * Add a `route` to `/register` and use the `register_get` controller to handle it
- Add the `askama`(**A**) dependecy to handle dynamic rendering of web pages
- Create a `models` package in `src` to handle objects:
    * Make a `mod.rs` which holds the package modules.
    * Create a `ui` module which will handle the web user interface.
        + Create a public structure `RegisterTemplate` to represent the register page.
        + Import the `Template` moule from `A` and implement it in the struct.
        + Specify the tempalte path to the register page. 
    * Add this module to the package list
    * Add this package to the `main.rs`
- Create a `controllers` package in `crate` to handle HTTP requests:
    * Make a `mod.rs` which holds the package modules
    * Add this package to the `main.rs`
    * Create a `auth.rs` which handle the authentication requests:
        + Import `HTTResponse` & `Responder` module from `AW`
        + Import the `Template` from `A` and `RegisterTemplate` from `crate/models/ui`
        + Create the `register_get` request handler function to provide register page:
            - Creates a new `RegisterTemplate`
            - Gives a `OK` Response to the client along with rendering the register template to client
    * Add this module to package list
- Using the register remplate in https://codepen.io/CrisD3v/pen/abPjQQv to create the register form
- Add the `templates` dir in the root directory to hold the html register template
    * Create the `register.html` file and copy html code from above, the form should have:
        + `username` field
        + `email` field
        + `password` field
    * Link this html file to the `RegisterTemplate` structure
- Add the `assets` folder in root directory to hold css, js and images
    * Create the `css` subdirectory
        + Add the `auth.css` stylesheet and link it to `register.html` file
- To server static files (assets) then:
    * Add `actix-files`(**AF**) dependency to render static files
    * Import the `Files` module from `AF`
    * Register a HTTP service to render the static files in `assets`
