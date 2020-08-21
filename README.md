# pe-rs-ist
A Twitter bot which tweets once a day about something.
It can easily be tweaked to make a bot which tweets daily about social causes.

## Setup
The bot uses [`egg-mode`](https://github.com/egg-mode-rs/egg-mode),
authentication is done the ["lazy" way](https://docs.rs/egg-mode/0.15.0/egg_mode/auth/index.html#shortcut-pre-generated-access-token) since it was developed with self-use in mind.

The tokens are stored in an `auth.toml` file which looks like:

```toml
API_KEY = "your-api-key"
API_KEY_SECRET = "your-api-secret"
BEARER_TOKEN = "your-bearer-token"
ACCESS_TOKEN = "your-access-token"
ACCESS_TOKEN_SECRET = "your-access-secret"
```

The bearer token is not required and can be left as an empty string (`""`).