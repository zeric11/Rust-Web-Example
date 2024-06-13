# Rust Web Example

Eric Zmitrovich

This repo contains my work for 
CS 510: Rust Web Development.

Spring 2024


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


## Homework 3 Update

For the front-end app, I cloned the knock-knock-yew repo and modified it
to work with the QnA server implemented in Homeworks 1 and 2.
The basic intended goal of the front-end was to display all
questions in the database and search for questions by ID.
Another API call was added to the back-end portion of this project to
accommodate the feature of retrieving a question by ID.

However, even though the back-end is functioning correctly and is
able to serve JSON data of questions from the database,
I could not get the yew app to successfully retrieve from the back-end.
Every call of the `get_question()` function in `qna.rs` results in a
"Failed to fetch" error. 
Unfortunately, I was not able to solve this issue in time for the due date.


## Sources

I followed the [Rust Web Development book's coding instructions](https://github.com/Rust-Web-Development/code) 
to write this program where I adapted the code to work with Axum according the work done in the class'
[knock-knock repo](https://github.com/pdx-cs-rust-web/knock-knock).
The front-end portion of this project was based on the
[knock-knock-yew repo](https://github.com/pdx-cs-rust-web/knock-knock-yew).