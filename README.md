# tom
A minimal command line tool for notifying the completion of your command.

### Build
```
cargo build --release
```

### Usage
```
./target/release/tom <path>
```
To create the `.tomrc` file in the user directory and configure the Pushover keys.
```
pushover_app_token = "THIS_IS_YOUR_APP_TOKEN"
pushover_user_key = "THIS_IS_YOUR_USER_KEY"
```
