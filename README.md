# Tasks
## Section A - Authentication

**NB**// All routes for authentication will have a scope of `/auth` 

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
### 8. Add Get controller for Login
#### Part 1 - Template
In `templates`, create the login html template based on register html template:
- identifier field
- password field
- error field
#### Part 2 - UI
In `ui` models add `LogInTemplate`:
- identifier field
- error field
#### Part 3 - get controller
Add 'login_get` in controllers module:
- It renders an empty `LogInTemplate`
#### Part 4 - login route
In `main` module:
- add `login` route which uses the `login_get` controller
- Import `login_get` controller from `auth`

In `register` html template, add the `/login` route

### 9. Add POST controller for login
#### Part 1 - Add dB operation to find user
Add `find_user` fn in `users` db operation:
- It accepts 2 parameter which is:
    * a string
    * A DatabaseConnection
- Add `regex` dependency to use regular expressions
- It finds whether the parameter is a user(id/name/email)
- Fetches the user based on the parameter type
- Returns a QueryResult

Add `verify_password` fn in `users` db operation:
- It accepts 2 parameter which is:
    * A `User` instance
    * A `password
- It compares if the password matches
- Returns a boolean value
#### Part 2 - Add form model for Login
Add `LoginForm` struct in `users` model to represent Login Form data:
- It has 2 fields:
    * `identifier`: Username/Email, is a string
    * `Password`: Is a string
#### Part 3 - Add post controller
Add `login_post` fn in `auth` controller:
- It extracts the login form data & App State
- All errors are handled by `login_error`
- It first attempts to get DB connection
- It then attempts to find the user based on the `identifier` field given
- Afterwards it verifies if the identified user has same password as one provided
- Once all above returns true it then calls the `handle_login` function
#### Part 4 - Add post route
In `main` module:
- add `login` route which uses the `login_post` controller
- Import `login_post` controller from `auth`

## Section B - Session Management
### AIM
- Once a user log in or register, a new session is created
- On server side:
    * Generates a new session id for user 
    * `redis` is used to store session id for user and respective data
    * Set expiration time to session 
### I. Server side
#### 0. Setup redis on machine
- Install the redis server:
```bash
$ sudo apt install redis-server
```
 - Change supervised state to systemd if running on ubuntu, on the following line
 ```bash
$ sudo grep -n "^supervised" /etc/redis/redis.conf
236:supervised systemd
```
- Add redis password as follows, by changing `foobared` to required password
```bash
$ sudo grep -n "^# requirepass" /etc/redis/redis.conf
739:# requirepass foobared
```
- Make sure the redis is bound to localhost:
```bash
$ sudo grep -n "^bind" /etc/redis/redis.conf
68:bind 127.0.0.1 ::1
```
- Restart redis and check to see if changes have been applied
```bash
$ sudo systemctl restart redis
$ sudo netstat -lnp | grep redis
tcp        0      0 127.0.0.1:6379          0.0.0.0:*               LISTEN      11160/redis-server  
tcp6       0      0 ::1:6379                :::*                    LISTEN      11160/redis-server  
```
- To test if one is able to enter and fetch data (replace `password` with you're password):
```bash
$ redis-cli
127.0.0.1:6379> auth <password>
OK
127.0.0.1:6379> set key1 10
OK
127.0.0.1:6379> get key1
"10"
127.0.0.1:6379> exit
$
```
#### 1. Setup redis connection pool
- Add `redis` url as follows, replacing `<password>` with you're redis password:
```bash
$ echo 'REDIS_URL=redis://:<password>@127.0.0.1/' >> .env
```
- Add `redis` dependency to store session data and `r2d2-redis` to handle redis connection pools
- Add `establish_redis_connection` in `connections` submodule to:
    * It fetches the `redis_url` from enviroment variables
    * It creates and returns a connection pool
- Add the `RedisPool` to the `AppState` structure and initialize it in the `app_state` in `main` fn

#### 2. Setup Session Models
For a session model, it would typically include the following fields:

- **Session ID** (`session_id`): A unique identifier for the session. This could be a string generated using a secure method.
- **User ID** (`user_id`): The ID of the user associated with the session.
- **Created At** (`created_at`): A timestamp indicating when the session was created.
- **IP Address** (`ip_address`): The IP address from which the session was initiated.
- **User Agent** (`user_agent`): The user agent string from the browser or device initiating the session.
These fields allow us to track and manage sessions effectively, including setting expiration times and associating sessions with users.
- Add the following implementations:
    * `new` that creates a session given the above values and has default duration of 30 minutes
    * serializa and deserialize fn for naive_datetime
#### 3. Fetch client ip & user agent
Create a `client_info` submodule in `utils` module to fetch info of the client
- Add `get_ip` to get client ip address (either from reverse proxy or direct connection) or return `unknown` if cannot get
- Add `get_browser` to get client browser or return `unknown` if cannot get

#### 4. Generate secure session ID 
- Add the `rand` dependency to generate random data
- Add the `session` submodule in `db_operations`
    * Add the `generate_session_id` function which returns a unique and secure session id
    * The session id must be 32 bytes long and consist of only alphanumerical
    * This process repeats until a unique session id is generated which doesnt exist in redis database, and returns the id

#### 4. Create and store session
- Create a `create_session` fn in the `db_operations::session` module that will:
    * Create a new session module
    * Store the session in the redis database with its id as its key and session object as json
    * Set it to expire after 30 minutes
    * Create a new cookie based on session
    * It should return the cookie

#### 5. Handle login controller
- In the `auth` controller add the `handle_login` fn which will:
    * Create a new session for the user
    * Send the `session_id` to user as cookie
    * Redirect to the `/dashboard`
#### 6. Check session
- Create a `check_session` fn in `db_operations::session`:
    * It will check for a given user whether session exists via cookie
    * Returns a boolean value

## Section C - Dashboard
###  I. Frontend
#### 0. Create dashboard page
- Create the `dashboard.html` & `dashboard.css` file in templates and assets respectively
- Create the `DashBoardTemplate` in `ui` models 
### II.  Backeend
#### 0. Serve dashboard page
- Add the `/dashboard` route in the `main`
- Create `dashboard` module controller
- In above controller, create the `dashboard_get` function:
    * To render dashboard page
    * Uses `check_session` to chek if session exists
    * If it does render dashboard page
    * Otherwise redirect user to `/auth/login`
# Resources
- [Postgress](https://www.cherryservers.com/blog/how-to-install-and-setup-postgresql-server-on-ubuntu-20-04)
- [Actix](https://actix.rs/docs/getting-started/)
- [Register template](https://codepen.io/CrisD3v/pen/abPjQQv)
- [diesel](https://diesel.rs/guides/getting-started)
- [bootstrap](https://getbootstrap.com/docs/4.0/layout/overview/)
- [Setup redis](https://www.digitalocean.com/community/tutorials/how-to-install-and-secure-redis-on-ubuntu-20-04)