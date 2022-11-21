# Runner

## 🤔 What is this?
A cli tool that let's you map commands to a shorter alias. Run the mapped command using the <ALIAS_NAME>.

```
runner <ALIAS_NAME>
```
You can map all your commands in runner.toml file

An example runner.toml file:
```toml
[Command]
alias = "ci"
program = "cargo"
args = ["install", "lsd"]

[Command]
alias = "nv"
program = "node"
args = ["-v"]

[Command]
alias = "cc"
program = "cargo"
args = ["check"]

```
### An example run for [ci] alias
### Output: 
[![ci](images/ci.gif)

## 📦 Installation
```
cargo install runner-cli
```

## 🏁 Initialize

#### It will initialize a empty runner.toml file in config directory

```
runner --init
```
### Output:
![init](images/init.gif)


## ⚙️ Config

#### Get the path to runner.toml file by running runner with config option
```
runner --config
```

## 🗺️ Mappings

#### Get all the user mapped alias

```
runner --mapping
```

### Output: 

![mapping](images/mapping.png)

## ➕ Add

#### Map a new command to an alias right from the terminal

```
runner --add
```

### Output: 

![add](images/add.gif)

## 🔍 Find

#### Find a command mapped to the given alias

```
runner --find <ALIAS_NAME>
```

### Output:

![find](images/find.gif)

## 🖥️ Options:

```
    -a, --add        Map a new command to a alias right from terminal
    -c, --config     Path of the the config file runner.toml
    -f, --find       Find a command mapped to the give alias
    -h, --help       Print help information
    -i, --init       Initialize a empty runner.toml file
    -m, --mapping    Show all the user defined mappings
    -V, --version    Print version information
```

## 📄 TOML file guide:

#### Each command starts wtih a [Command] key


```toml
[Command]
alias =  "Shorter subcommand you want to use instead of the command"
program = "CLI Program Name(like: git, cat, batcat, code, neofetch, cargo, python, node, npm etc.)"
args = "Arguments you want to pass to the cli program"
```

## Contribution
#### Have any suggestion or feature idea/request feel free to open a issue.
