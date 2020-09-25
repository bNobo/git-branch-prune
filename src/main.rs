use std::env;
use std::io::{self, BufRead};
use std::process::Command;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let force = args.contains(&String::from("--force"));

    println!("executing 'git remote prune origin'...");

    let prune = Command::new("cmd")
        .arg("/C")
        .arg("git remote prune origin")
        .output()
        .expect("aie");

    let prune_std = String::from_utf8(prune.stdout).expect("aie");
    let prune_err = String::from_utf8(prune.stderr).expect("ouille");

    if prune_std.ne("") {
        println!("{}", prune_std);
    }

    if prune_err.ne("") {
        println!("{}", prune_err);
    }

    if !prune.status.success() {
        panic!("Unexpected error");
    }

    println!("executing 'git branch'...");

    let local_branches = Command::new("cmd")
        .arg("/C")
        .arg("git branch")
        .output()
        .expect("failed to execute process");

    let local_branches_std = String::from_utf8(local_branches.stdout).expect("aie");
    let local_branches_err = String::from_utf8(local_branches.stderr).expect("ouille");

    if local_branches_err.ne("") {
        println!("{}", local_branches_err);
    }

    if !local_branches.status.success() {
        panic!("Unexpected error");
    }

    let local_branches = local_branches_std.split("\n");

    println!("executing 'git branch --remote'...");

    let remote_branches = Command::new("cmd")
        .arg("/C")
        .arg("git branch --remote")
        .output()
        .expect("failed to execute process");

    let remote_branches_std = String::from_utf8(remote_branches.stdout).expect("aie");
    let remote_branches_err = String::from_utf8(remote_branches.stderr).expect("ouille");

    if remote_branches_err.ne("") {
        println!("{}", remote_branches_err);
    }

    if !remote_branches.status.success() {
        panic!("Unexpected error");
    }

    let remote_branches = remote_branches_std.split("\n");

    let mut remotes = Vec::new();

    for remote_branch in remote_branches {
        //println!("remote: {}", remote_branch);
        remotes.push(remote_branch.trim_start());
    }

    let mut to_delete = Vec::new();

    for local_branch in local_branches {
        if local_branch != "" && !exists_on_remote(&remotes, local_branch) {
            to_delete.push(local_branch.trim_start());
        }
    }

    println!("You are about to delete these local branches:");

    for branch in to_delete.clone() {
        println!("{}", branch);
    }

    println!("Are you sure ? (y, N)");

    let stdin = io::stdin();
    let mut resp = String::new();
    stdin.lock().read_line(&mut resp).expect("ouille");

    if resp.to_uppercase().starts_with("Y") {
        for branch in to_delete {
            delete_local_branch(branch, force);
        }
    } else {
        println!("Cancelled by user");
    }
}

fn exists_on_remote(remotes: &Vec<&str>, branch: &str) -> bool {
    let mut remote_counterpart = String::from("origin/");
    let mut branch = branch;

    if branch.starts_with("*") {
        branch = &branch[1..];
    }

    branch = branch.trim_start();

    remote_counterpart.push_str(branch);
    //let remote_counterpart = &remote_counterpart[..];
    // println!(
    //     "branch = {}, remote_counterpart = {}",
    //     branch, remote_counterpart
    // );

    return remotes.contains(&&remote_counterpart[..]);
}

fn delete_local_branch(branch: &str, force: bool) {
    let d_arg: &str;

    if force {
        d_arg = "-D";
    } else {
        d_arg = "-d";
    }

    println!("executing 'git branch {} {}'...", d_arg, branch);

    let delete = Command::new("cmd")
        .arg("/C")
        .arg("git branch")
        .arg(d_arg)
        .arg(branch)
        .output()
        .expect("aie");

    let standard = String::from_utf8(delete.stdout).expect("aie");
    let error = String::from_utf8(delete.stderr).expect("ouille");

    if standard.ne("") {
        println!("{}", standard);
    }

    if error.ne("") {
        println!("{}", error);
        println!("There was an error while delete branch. Try with 'git-branch-prune --force' to make a 'git branch -D'");
    }
}
