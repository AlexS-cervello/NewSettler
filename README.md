# NewSettler
Telegram bot for pulling news from https://news.ycombinator.com/


## Getting Started

These instructions will get you a copy of the project up and running on your local machine

### Usage

Clone the repository
```
git clone https://github.com/Feston229/NewSettler.git
```

Navigate into the `NewSettler` directory
```
cd NewSettler
```

Export env variables
```
export BOT_TOKEN=<TOKEN>
export SETTLER_DB=<DB_PATH>
```

You can create db with
```
touch db.sqlite
```

Build
```
cargo build --release
```

Run
```
./target/release/new-settler-bot
```

## License

This project is licensed under the GNU GPLv3 License - see the [LICENSE](LICENSE) file for details

