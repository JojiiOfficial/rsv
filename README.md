# rsv
The runit sv command rewritten in rust with additional features.

# Additional features
- Enable/Disable services (automatically creating a symlink)
- Listing services
- Custom timeout
- Much cleaner code than the original sv command

# Installation
### Manually
Run `make release` and `sudo make install`.<br>

### Artix linux (AUR)
`yay -S rsv` (Other AUR helpers will work too)

# Usage
```txt
rsv 1.1.0
A tool to maintain runit services like systemd services.

USAGE:
    rsv [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

OPTIONS:
    -t, --timeout <timeout>

SUBCOMMANDS:
    alarm        Send SIGALARM if service is running
    continue     Send SIGCONT if service is running
    disable      Disable a service
    enable       Enable a service
    help         Prints this message or the help of the given subcommand(s)
    hup          Send SIGHUP if service is running
    interrupt    Send SIGINT if service is running
    kill         Send SIGKILL if service is running
    list         List services
    once         Start if service is not running. Do not restart if it stops
    pause        Send SIGSTOP if service is running
    restart      Restart a service
    start        Start a service
    status       Status a service
    stop         Stop a service
    term         Send SIGTERM if service is running
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
- [ ] Shell completion
- [ ] Improving Envar usage (Fix multiuser support)
