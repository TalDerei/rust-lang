# Lehigh University CSE262 - Programming Languages - Homework 2

This assignment asks you to familiarize yourself with the syntax and semantics of the Rust programming language. The assignment is broken down into 7 sections:

- Section1 - Variables
- Section2 - Strings
- Section3 - Primitive Types
- Section4 - Control
- Section5 - Functions
- Section6 - Structs
- Section7 - Move Semantics

Each section presents a number of problems that ask you to correct the given code so that it compiles. Do this for every file so that the project builds and all tests pass. Each section contains a README.md file with a link to the relevant chapters in the Rust programming language handbook. If you're stuck, be sure to look at the hint at the bottom of each file.

## Running

The functions you are to fix in this project are annotated as `#[test]`, which means they only run in a testing context. To run the project:

```bash
> cargo test
```

This will run every test defined throughout the project. You can test a specific module though. For example, if you only wanted to test the variables module you could run the following command:

```bash
> cargo test variables
```

Or to test specifically one file:

```bash
> cargo test variables::variables1
```

However, because the files in this project contain a number of syntax errors, these will all be reported (from every module) even though you have only specified to run a specific test module. If you would like to filter out these other errors while you are working on a single project, you can comment out the modules in src/lib.rs.

## Instructions

1. Fork the relevant repository into your own namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Set your forked repository visibility to private. [Instructions](https://docs.gitlab.com/ee/public_access/public_access.html#how-to-change-project-visibility)
3. Add user "LehighCSE262" as a member to the project with "maintainer" access-level. [Instructions](https://docs.gitlab.com/ee/user/project/members/#add-a-user). 
4. Clone your newly forked repository. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
5. Write code sufficient to make the tests pass. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work.
6. When you've passed all tests and you're ready to submit your work, indicate this on the corresponding assignment on course site. 

