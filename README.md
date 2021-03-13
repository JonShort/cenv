# cenv - environment variable file changer
[![Continuous integration](https://github.com/JonShort/cenv/actions/workflows/tests.yml/badge.svg)](https://github.com/JonShort/cenv/actions/workflows/tests.yml)

![usage](https://user-images.githubusercontent.com/21317379/111026089-60825800-83e0-11eb-99ab-054463749377.gif)

Using a comment pattern in your .env files, easily swap between envs in local development.

## Usage

1. Add the "cenv" pattern to your .env file
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000
# REQUIRE_LOGIN=false

# ++ live ++
# API_ADDRESS=https://myliveapi.com
# REQUIRE_LOGIN=true
```

2. Run cenv, choosing an env keyword to use
```bash
cenv live
```

3. Check your .env, the keyworded env vars will now be uncommented
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000
# REQUIRE_LOGIN=false

# ++ live ++
API_ADDRESS=https://myliveapi.com
REQUIRE_LOGIN=true
```
