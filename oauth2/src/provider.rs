struct provider {
    client_id: &str,
    client_secret: &str,
    auth_url: &str,
    token_url: &str,
}

pub const github: provider = provider{
    client_id: "cf07a51bf0c95a35fff8",
    client_secret: "89969eb236953ca9c0c86de8064f2d91fb69b14f",
    auth_url: "https://github.com/login/oauth/authorize",
    token_url: "https://github.com/login/oauth/access_token",
}

