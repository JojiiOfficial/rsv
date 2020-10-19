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
rsv 1.1.0
Jojii S
A tool to maintain runit services like systemd services

USAGE:
    rsv [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    
    -V, --version    Prints version information

OPTIONS:
    -t, --timeout <timeout>    

SUBCOMMANDS:
    list         List services
    enable       Enable a service
    disable      Disable a service
    start        Start a service
    stop         Stop a service
    restart      Restart a service
    kill         Send SIGKILL if the service is running
    pause        Send SIGSTOP if the service is running
    continue     Send SIGCONT if the service is running
    alarm        Send SIGALARM if the service is running
    help         Prints this message or the help of the given subcommand(s)
    hup          Send SIGHUP if the service is running
    interrupt    Send SIGINT if the service is running
    once         Start if service is not running. Do not restart if it stops
    status       Get the status of a service
    term         Send SIGTERM if the service is running
```

# Examples
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
- [x] Auto sudo
