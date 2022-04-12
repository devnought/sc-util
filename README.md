# sc-util
Utility to manage the Star Citizen shader caches and user folder.

```
sc-util 1.0.0

USAGE:
    scu.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    clean         Clear user folder or shaders
    config        Configure Star Citizen root path
    create-cfg    Create fresh user config file
    help          Print this message or the help of the given subcommand(s)
```

## Why was this created?
To simmplify cleaning up your shader/user folders for multiple environemnts, without having to navigate to those respective locations to do so.

It only saves a little bit of time, but if you do it frequently, that time can add up. Only you can decide if it's worthwhile.


## How to use
NOTE: As of 3.17, this first step is only required for managing the `USER` folder and creating an option fresh `USER.cfg` file, as the shader cache was moved to `appdata/local`.

### 1. Configure the root Star Citizen install location
Run the command `scu config set <PATH-TO-STAR-CITIZEN-ROOT>`
```
scu config set "C:\Program Files\Roberts Space Industries\StarCitizen"
```

You can verify or check your set location at any time with `scu config view`
```
PS C:\> scu config view
C:\Program Files\Roberts Space Industries\StarCitizen
```

This will become the base directory where commands will look for your `LIVE` or `PTU` folders.

### 2. Get cleaning!
Cleaning is split across two commands for `shaders` and the `userfolder`:
```
scu clean shaders
```
```
scu clean userfolder <ENVIRONMENT>
````
Where `<ENVIRONMENT>` can be `PTU` or `LIVE`.

### 3. Creating a `USER.cfg` file
While testing on the PTU, it's handy to always see the `SessionInfo` QR code, along with performance information. If you want those to be displayed by default, the relevant display commands can be added to this file.

If you don't want to create one by hand, I got you covered:
```
scu create-cfg -e <ENVIRONMENT>
```
Where `<ENVIRONMENT>` can be `PTU` or `LIVE`.
