# rust-forum

Forum software written in rust with support for [OpenID
Connect](https://openid.net/connect/) for authentication.

# Development

Use the provided example configuration for [dex](https://dexidp.io/) to start a
local OpenID Connect provider, then start rust-forum.

```
dex serve dex_example.yaml
cargo watch -x run
```

