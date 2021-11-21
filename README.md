# fdb-cli
A command-line tool to make developing with FoundationDB easier with protobufs. 

## Installation
N/A

## Usage
```sh-session
fdb-cli [command]

fdb-cli [command] help
```

## Commands

- [`setup`](#setup)
- [`get`](#get)
- [`delete`](#delete)
- [`reset`](#reset)
- [`move`](#move)

### Setup
```sh-session
fdb-cli setup --cluster-file /etc/foundationdb/fdb.cluster
```

### Get
#### Get a singular kv pair
```sh-session
fdb-cli get key users/259e748b-b9c6-48de-b366-24d2af598e63
```

#### Get a range of kv pairs
```sh-session
fdb-cli get range --start users/100 --end users/500
```

### Delete
#### Delete a singular kv pair
```sh-session
fdb-cli delete key users/259e748b-b9c6-48de-b366-24d2af598e63
```

#### Delete a range of kv pairs
```sh-session
fdb-cli delete range --start users/100 --end users/500
```

### Reset
```sh-session
fdb-cli reset
```

### Move
WIP
