# rsv
The runit sv command rewritten in rust with nice new features.

# Additional features
- Enable/Disable services (automatically creating the symlink)
- Bash completion
- Listing services
- Custom timeout
- Much cleaner code than the original sv command
- Automatically use sudo if ran as user (feature: `auto_sudo`. Used by default)

# Installation

### From my [pacman repository](https://repo.jojii.de)
(Add the repo like described)<br>
`pacman -S rsv`

### AUR
`yay -S rsv` (Other AUR helpers will work too)

### From [crates.io](https://crates.io/crates/rsv)
`cargo install rsv`

### From git
```
git clone https://github.com/JojiiOfficial/rsv
cd rsv
cargo build --release
```

# Usage
```txt
rsv 1.4.0
Jojii S
A tool to maintain runit services like systemd services

USAGE:
    rsv [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose
    -V, --version    Prints version information

OPTIONS:
        --generate <generator>    Generate completion scripts for a given type of shell [possible
                                  values: bash, elvish, fish, zsh]
    -t, --timeout <timeout>

SUBCOMMANDS:
    alarm        Send SIGALARM if the service is running
    continue     Send SIGCONT if the service is running
    disable      Disable a service
    enable       Enable a service
    help         Prints this message or the help of the given subcommand(s)
    hup          Send SIGHUP if the service is running
    init         Generate configuration using information about system processes
    interrupt    Send SIGINT if the service is running
    kill         Send SIGKILL if the service is running
    list         List services
    once         Start if service is not running. Do not restart if it stops
    pause        Send SIGSTOP if the service is running
    restart      Restart a service
    start        Start a service
    status       Get the status of a service
    stop         Stop a service
    term         Send SIGTERM if the service is running
```

# Examples
```bash
sudo rsv init # generate user configuration file for rsv
```

```bash
sudo rsv list --disabled/--enabled # list all disabled/enabled services 
```

```bash
sudo rsv list --enabled --down # list all enabled services which aren't running
```

```bash
sudo rsv enable cupsd # enabled cupsd
```

```bash
sudo rsv start cupsd # start cupsd service (enable if service is disabled)
```

# TODO
- [x] Listing services
- [x] Shell completion
- [] Auto sudo
