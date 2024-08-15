## Tasks
### 0. Create rust project
- Install rust language and create the `loansasa` project as follows:
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ cargo new loansasa
$ cd loansasa
```
### 1. Add actix server
- Add `actix-web`(**AW**) dependency in `.toml` to act as web application server
- In the main async fn in main.rs:
    * Import `HttpServer` from `AW` and start a new instance
    * Bind to loopback address (**127.0.0.1**) port `8080`, run it and await

### 2. Add register GET route and register page
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
### 3. Nginx configuration
- Install nginx as the web server:
```bash
$ sudo apt-get install nginx
```
- Add a domain name mapper to localhost IP address in `/etc/hosts`:
```bash
127.0.0.1   loansasa.com
```
- Add the nginx server block and configure it as a reverse proxy as follows:
```bash
$ sudo cat /etc/nginx/sites-available/loansasa
server {
	listen 80;

	server_name loansasa.com;

	location / {
		proxy_pass http://127.0.0.1:8080;  # The address of your backend server
	        proxy_set_header Host $host;
	        proxy_set_header X-Real-IP $remote_addr;
	        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
	        proxy_set_header X-Forwarded-Proto $scheme;		
	}
}
$ sudo ln -s /etc/nginx/sites-available/loansasa /etc/nginx/sites-enabled/
$ sudo nginx -t
nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
nginx: configuration file /etc/nginx/nginx.conf test is successful
$ sudo systemctl reload nginx
```
- Now to be able to access the website locally, in the web browser type:
`http://loansasa.com/register`

### 4. Set up loansasa Postgress Database
- Install the database:
```bash
$ sudo apt install postgresql postgresql-contrib
```
- Log in as root user:
```bash
$ sudo -u postgres psql
psql (14.12 (Ubuntu 14.12-0ubuntu0.22.04.1))
Type "help" for help.

postgres=# 
```
- Create a postgress user's account as well as loansasa database:
```bash
postgres=# create database loansasa;
CREATE DATABASE
postgres=# create user <USERNAME> with encrypted password '<PASSWORD>';
ERROR:  role "razaoul" already exists
postgres=# GRANT ALL PRIVILEGES ON DATABASE loansasa TO razaoul;
GRANT
postgres-# 
```
- Install diesel cli:
```bash
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh
```
- Setup the database url to the database as follows:
```bash
$ echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
$ diesel setup
```
- The above sets up the diesel database and creates the migrations folder to handle tables
- Next create the users table to contain the following attributes:
    * `id` : is the serial primary key
    * `name`: The username of the user
    * `email`: The email of the user
    * `password`: The password the user
    * `created_at`: The time the user created their account
```bash
$ diesel migration generate create_users
$ diesel migration run
```
### 5. Set up the database connection pool
- Instead of establishing new connections database connections each time, a `connection pool` is used
where:
    * Each time the application starts, the pool is initialized with a predetermined number of db connections
    * When application needs to interact with the database it borrows a connection from the pool
    * Uses this connection to perform db operations and returns it upon completion
- Add the `diesel` dependecy which will have the `postgres` and `r2s2` features to handle connection pool and db operations
- Create the `db_operations` package to handle all db operations:
    * Add this package to main module.
    * Create the `establish_connection` fn in `db` module to create the connection pool:
        + It fetches the database url
        + Creates a connection manager
        + Creates a connection pool having max number of connections of 15
- Create the `AppState` model in `app_state` to set up a shared piece of data:
    * Currently it will only have the DbPool from `db`
    * Add it to `models` list
- Next in `main` module:
    * Initialize the connection pool
    * Create `AppState` with the connection pool
    * Share the `AppState` with all routes
### 6. Create users model to represent user table
- Create the users model to represent users table:
    * `id`: positive 32 bit integer
    * `name`: String
    * `email`: String
    * `password`: String which is hashed
    * `created_at`: Should be a date time:
        + Add `chrono` dependency to use DateTime, as part of `diesel` features
        + Use NaiveDateTime since the timezone will be local hence wont matter
- Create a `NewUser` model in `users` for creating users with default values:
    + `name`: String
    + `email`: String
    + `password`: String which is hashed
    + All field are references since diesel requires insertables to be so
- Create a `RegisterForm` model in `users` to represent client register details:
    + Add `serde` dependency to deserialize form data
    + Same fields as `NewUser`but actual data type
### 7. Add db operations for Users to register
- Create a `users` module responsible for having all methods of users:
    * Add a `create_user` fn which:
        + Takes 2 parameters, the `NewUser` reference & Pgconnection
        + Inserts user into dB
        + Returns the QueryResult

### 8. Add POST controller for register
#### Part 1
In `auth` controller module add the `register_post` fn:
- Extracts the form data & App state
- Creates a `NewUser` based form data
- Attempts get DB Connection from pool
- If Successful then:
    * Attemps to add user details to DB
    * If successful:
        + Uses `handle_login` fn to redirect user to dashboard.
    * Else:
        + Uses `register_error` fn to handle this error
- Else: 
    * Uses `register_error` fn to handle this error
#### Part 2
In `auth` controller module add the `register_error` fn to handle errors during registration:
- It accepts:
    1. the failed user creation info,
    2. Error message
    3. the HTTP status 
- It renders the register template based above info
- It returns a HTTP Response bases on the status code
## Resources
- [Postgress](https://www.cherryservers.com/blog/how-to-install-and-setup-postgresql-server-on-ubuntu-20-04)
- [Actix](https://actix.rs/docs/getting-started/)
- [Register template](https://codepen.io/CrisD3v/pen/abPjQQv)
- [diesel](https://diesel.rs/guides/getting-started)