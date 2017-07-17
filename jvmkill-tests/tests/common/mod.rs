/*
 * Copyright (c) 2015-2017 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn run_java(class: &str, arguments: &str, expected_stdout: &[&str], expected_stderr: &[&str]) -> bool {
    let output = Command::new(&java())
        .arg(format!("-agentpath:{}{}", jvmkill().to_str().unwrap(), arguments))
        .arg("-cp").arg(jvmkill_test().to_str().unwrap())
        .arg("-Xmx5m")
        .arg(class)
        .output().expect("failed to run Java process");

    assert_contents(&output.stdout, expected_stdout);
    assert_contents(&output.stderr, expected_stderr);

    output.status.success()
}

fn assert_contents(stream: &Vec<u8>, expected :&[&str]) {
    let s = String::from_utf8_lossy(stream);
    println!("OUTPUT:\n{}\n:OUTPUT", s);
    let mut success = true;
    for o in expected {
        if !s.contains(o) {
            println!("{}", o);
            success = false;
        }
    }
    if !success {
        println!("the above were not found in:\n{}", s);
    }
    assert!(success);
}

fn java() -> PathBuf {
    PathBuf::from(env::var("JAVA_HOME").unwrap()).join("bin").join("java")
}

fn jvmkill_test() -> PathBuf {
    env::current_dir().unwrap().parent().unwrap().join("resource-exhaustion-generator").join("target").join("resource-exhaustion-generator-0.0.0.jar")
}

fn jvmkill() -> PathBuf {
    let root = PathBuf::from(env::var("LD_LIBRARY_PATH").or(env::var("DYLD_LIBRARY_PATH")).unwrap().split(":").next().unwrap());

    if cfg!(target_os = "macos") {
        root.join("libjvmkill.dylib")
    } else {
        root.join("libjvmkill.so")
    }
}
