# RP_judge
This is a program, that checks student HW submissions for correct submission structure. The program extracts all repository links from students that submitted a link to the repo. For each extracted and cloned repo, the program will check if the repo contains filders called `Task1` and `Task2`, as instructed to the students. 

After it will check how many commits are present that affect each task. For each commit the program will try to compile the `main.c` file, that should be present in each folder. Compilation of the commit is considered successful if the `gcc` compilation runs with no output to stdout (no errors, no warrnings).

The program will count how many compilations were successful per task. 

After performing all the compilations, the program will create a `results.xlsx` file, that contains the summary of the performed tests. The table will contain the following columns:
- student_folder: `string`
- git_repo: `string`
- cloned: `boolean`
- has_task1: `boolean`
- has_task2: `boolean`
- commits_task1: `number`
- commits_task2: `number` 
- all_commits_compile_task1: `boolean`
- all_commits_compile_task2: `boolean`
- final_commit_compile_task1: `boolean`
- final_commit_compile_task2: `boolean`
- successful_compiles_task1: `number`
- successful_compiles_task2: `number`

## Prerequisites
1. [Rust](https://www.rust-lang.org/tools/install) programming language (developed on 1.64.0)
2. `llvm` for parsing xlsx export (`sudo apt install llvm llvm-dev libclang-dev `)

## Setup
1. Clone the project
```
git clone https://github.com/VakeDomen/RP_judge.git
```
2. Move into the project
```
cd RP_judge
```
2. Compile the project
```
cargo build --release
```

Once you build the project, the binary is located in the folder `taget/release/rp_judge` and you can move it wherever.

## Running
The program accepts arguments, that are paths to `*.zip` files, that are downloaded from the moodle. The  zip (if downloaded from moodle usig `Download all submissions` button) should contain multiple folders, that are submissions of students. Each submission folder should contain a HTML file named `onlinetext.html`. This is done automatically, if the submission is of type "online text" (submission types setting in HW settings on moodle).

Once you download all the submissions, you can run the program.
Let's assume you downloaded 2 `*.zip` files, one named `RP_slo.zip` and one `RP_en.zip` and are contained in the same folder as the `rp_judge` binary. You can run the programm as follows:

```
./rp_judge RP_slo.zip RP_en.zip
```
The program will create a `rp_workspace` folder, where you can find all the extracted submissions, submitted repositories and a `results.xlsx` file, that contains a summary of checks performed on the homeworks.

