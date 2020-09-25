# git-branch-prune

A CLI tool to prune local GIT branches that are not present on remote.

## Prerequisites

* GIT CLI should be installed because this tool depends on it.
* Works only under Windows for the moment

## How to build it ?

First you have to install Rust. Checkout the documentation here: https://doc.rust-lang.org/book/ch01-01-installation.html

Then you can build using cargo:

```shell
cargo build
```

## How to run it ?

You have to execute the tool in a GIT repository folder. 

`git-branch-prune` will prune every local branch that do not have a remote counterpart.

You should place the executable in a folder that is a member of %PATH% so you can easily execute it from any repository.

By default it will not delete branches that are not fully merge. To force deletion you can use the "--force" argument:

`git-branch-prune --force`

## How it works ?

The tool first executes `git remote prune origin` to clean local repository from remotes that do not exist anymore. Then it makes a comparison between remote an local branches and make a `git branch -d <branch_name>` on every local branch that do not have a remote counterpart. Specifying "--force" argument force the deletion by replacing "-d" with "-D".