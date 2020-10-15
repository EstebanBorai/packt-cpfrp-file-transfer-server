<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  </div>
  <h1 align="center">simple-file-transfer-server</h1>
  <h4 align="center">
    A simple Actix Web Server which performs CRUD operations on the File System.
  </h4>
</div>

## Motivation

Write a REST API with Actix capable of performing CRUD operations on the file system using `actix-web` and `futures`.

This approach is taken due to interest on understanding Rust Futures and getting to know the API better, current Actix
versions make use of the `actix-web-rt`. Instead this version relies on `futures` to accomplish asynchronous programming.

The idea is inspired on the `file_transfer` project explained in "Creative Projects for Rust Programmers" by Carlo Milanesi,
but as the project evolved major changes were made in order to experiment other Rust features which are out of the scope
of the book.

## Running Locally

```bash
# clone the repository
git clone https://github.com/EstebanBorai/simple-file-transfer-server.git

# step into project directory
cd ./simple-file-transfer-server

# run with cargo
cargo run
```

## Endpoints

> Every filesystem operation will run in the included `archive` directory, if a request is send as `/hello_world.txt` then the path to the file in question will be `$PWD/archive/hello_world.txt`.

Method | URI | Description | Req. Body | Res. Body
--- | --- | --- | --- | ---
**GET** | `/:filename` | Reads the file specified in the path | N/A | `String`
**POST** | `/:filename` | Creates a new file with the contents of the request body | `String` | `String`
**PUT** | `/:filename` | Overwrites a file with the contents of the request body | `String` | `String`
**DELETE** | `/:filename` | Removes the file specified in the path | N/A | N/A

## Contributions

Any contribution to this project is welcome, as pointed out above this project is inspired by
a book example but some changes where made to acomplish an universal usage and also to
experiment other features which are out of the scope of the book examples.

## License

Licensed under the GNU General Public License
