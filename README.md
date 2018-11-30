# OpenGM
Tools for GMs and DMs!

# Development Setup
1. Install sqlite3 libsqlite3-dev
    * `$ apt-get install sqlite3 libsqlite3-dev`

2. Create `.env` file in project root
    * set __OPENGM_DATABASE_URL__ to a local location (project_root/local folder is a good idea)

3. [Re]Generate documentation
    * `$ cargo doc --no-deps`
