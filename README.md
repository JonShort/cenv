# cenv - environment variable file changer

Using a comment pattern in your .env files, easily swap between envs in local development.

## Usage

1. Add the "cenv" pattern to your .env file
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000

# ++ live ++
# API_ADDRESS=https://myliveapi.com
```

2. Run cenv, choosing an env keyword to use
```bash
cenv live
```

3. Check your .env, the keyworded env vars will now be uncommented, and an "(active)" flag will appear on the chosen keyword
```.env
VARIABLE_1=something

# ++ local ++
# API_ADDRESS=http://localhost:5000
# REQUIRE_LOGIN=false

# ++ live ++ (active)
API_ADDRESS=https://myliveapi.com
REQUIRE_LOGIN=true
```
