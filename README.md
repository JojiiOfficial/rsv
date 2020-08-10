# rsv
The runit sv command rewritten in rust with additional features.

# Additional features
- Enable/Disable services (automatically creating a symlink)
- Custom timeout
- Much cleaner code than the original sv command

# Usage
```bash
rsv 0.1.0
A tool to maintain runit services like systemd services

USAGE:
    rsv [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    

OPTIONS:
    -t, --timeout <timeout>    

SUBCOMMANDS:
    disable    Disable a service
    enable     Enable a service
    help       Prints this message or the help of the given subcommand(s)
    reload     Reload a service
    restart    Restart a service
    start      Start a service
    status     Status a service
    stop       Stop a service
```