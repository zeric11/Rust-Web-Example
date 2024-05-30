# Rust Web Example

Eric Zmitrovich

This repo contains my work for 
CS 510: Rust Web Development.


## Homework 1 Update

The web server is able to run and the REST API as shown in the book has been implemented. 
The `get_questions` call will show the `questions.json` file in the brower.
However, since `QABase` is not yet completed, 
the put, post, and delete calls do not effect the `questions.json` file.


## Homework 2 Update

This application now connects to a PostgreSQL instance which allows 
it to store questions and answers into a persistent database.
The question and answer tables were added created according to listings 
7.4 and 7.6 from the Rust Web Development book.
The program expects there to be a user named "rustweb" with the password "postgres".


## Sources

I followed the [Rust Web Development book's coding instructions](https://github.com/Rust-Web-Development/code) 
to write this program where I adapted the code to work with Axum according the work done in the class'
[knock-knock repo](https://github.com/pdx-cs-rust-web/knock-knock).