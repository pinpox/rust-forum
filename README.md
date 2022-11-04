# rust-forum

Forum software written in rust with support for [OpenID
Connect](https://openid.net/connect/) for authentication.

![image](https://user-images.githubusercontent.com/1719781/199956062-460d8729-4a54-49c9-b1e5-5c9e743179eb.png)


# Development

Use the provided example configuration for [dex](https://dexidp.io/) to start a
local OpenID Connect provider, then start rust-forum.

```
dex serve dex_example.yaml
cargo watch -x run
```

