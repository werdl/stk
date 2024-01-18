use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{Error, Write};

macro_rules! command {
    ($howmany: expr) => (format!("{}", "$".repeat($howmany)).as_str())
}

fn main() {
    let input = env::args().nth(1).expect("Please provide a command");

    let mut output =
        String::from("
#include <stdio.h>

int main() {
char array[30000] = {0};
char *p = array;
");

    output.push_str(
        &input.split(".")
        .map(|x| 
        x.replace(command!(8), "}")
        .replace(command!(7), "int dup = 2**p;p++;while (2*dup) {dup--;")
        .replace(command!(6), "*p = getchar();")
        .replace(command!(5), "putchar(*p);")
        .replace(command!(4), "p--;")
        .replace(command!(3), "p++;")
        .replace(command!(2), "(*p)--;")
        .replace(command!(1), "(*p)++;")).collect::<Vec<String>>().join("\n"));

    output.push_str("\n}");

    let mut file = File::create("temp.c").expect("create failed");

    file.write_all(output.as_bytes()).expect("write failed");

    let o = Command::new("gcc")
    .arg("temp.c")
    .arg("-o")
    .arg("res")
    .output()
    .expect("failed to execute process");
}
