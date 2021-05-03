# cenv - environment variable file changer
[![Continuous integration](https://github.com/JonShort/cenv/actions/workflows/tests.yml/badge.svg)](https://github.com/JonShort/cenv/actions/workflows/tests.yml)

![usage](https://user-images.githubusercontent.com/21317379/111026089-60825800-83e0-11eb-99ab-054463749377.gif)

Using a comment pattern in your .env files, easily swap between envs in local development.

## Installation

### macOS

Install via. [homebrew](https://brew.sh/)

1. Connect to the [cenv tap](https://github.com/JonShort/homebrew-cenv)
```bash
brew tap jonshort/cenv
```

2. Install `cenv` from the tap
```bash
brew install cenv
```

### Windows (untested)

Download the latest binary from the [releases page](https://github.com/JonShort/cenv/releases)

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

## Releasing a new version

1. Ensure [CHANGELOG.md](https://github.com/JonShort/cenv/blob/main/CHANGELOG.md) is updated on main branch
2. Push a tag to origin matching the version referenced in the changelog
3. Follow the instructions in [homebrew-cenv](https://github.com/JonShort/homebrew-cenv) to ensure the new version is available via. homebrew
