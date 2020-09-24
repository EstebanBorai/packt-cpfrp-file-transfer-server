<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  </div>
  <h1 align="center">packt-cpfrp-file-transfer-server</h1>
  <h4 align="center">
    Actix FileSystem Server inspired in "file-transfer" project from "Creative Projects for Rust Programmers" by Carlo Milanesi
  </h4>
</div>

## Motivation

Write a REST API with Actix capable of reading, creating, deleting and updating files using `actix-web` and `futures`.

This approach is taken due to interest on understanding Rust Futures and getting to know the API better, current Actix
versions make use of the `actix-web-rt`. Instead this version relies on `futures` to accomplish asynchronous programming.
