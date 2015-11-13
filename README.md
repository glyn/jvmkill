# Overview

**jvmkill** is a simple [JVMTI][] agent that forcibly terminates the JVM
when it is unable to allocate memory or create a thread. This is important
for reliability purposes: an `OutOfMemoryError` will often leave the JVM
in an inconsistent state. Terminating the JVM will allow it to be restarted
by an external process manager.

[JVMTI]: http://docs.oracle.com/javase/8/docs/technotes/guides/jvmti/

It is often useful to automatically dump the Java heap using the
`-XX:+HeapDumpOnOutOfMemoryError` JVM argument. This agent will be
notified and terminate the JVM after the heap dump completes.

A common alternative to this agent is to use the
`-XX:OnOutOfMemoryError` JVM argument to execute a `kill -9` command.
Unfortunately, the JVM uses the `fork()` system call to execute the kill
command and that system call can fail for large JVMs due to memory
overcommit limits in the operating system.  This is the problem that
motivated the development of this agent.

# Building

```shell
make JAVA_HOME=/path/to/jdk
```
or just `make` if `JAVA_HOME` is set.

# Usage

Run Java with the agent added as a JVM argument:

```shell
-agentpath:/path/to/libjvmkill.so
```

Alternatively, if modifying the Java command line is not possible, the
above may be added to the `JAVA_TOOL_OPTIONS` environment variable.

# Testing

A test Java program `JvmKillTest.java` is supplied which can be compiled and run with:
```shell
make test
```
which allocates memory 1MB at a time, until it runs out. Make will run this with a small (5MB) heap, and the output should be something like this:

```
gcc -Wall -Werror -fPIC -shared \
        -I"$JAVA_HOME/include" \
        -I"$JAVA_HOME/include/darwin" \
        -o libjvmkill.so \
        jvmkill.c
chmod 644 libjvmkill.so
$JAVA_HOME/bin/javac JvmKillTest.java
$JAVA_HOME/bin/java -Xmx5m \
        -XX:+HeapDumpOnOutOfMemoryError \
        -XX:OnOutOfMemoryError='/bin/echo hello' \
        -agentpath:/full/path/to/libjvmkill.so \
        -cp /full/path/to/jvmkill JvmKillTest
triggering OutOfMemmory...
list size: 1
list size: 2
list size: 3
java.lang.OutOfMemoryError: Java heap space
Dumping heap to java_pid80427.hprof ...
Heap dump file created [4050248 bytes in 0.018 secs]
#
# java.lang.OutOfMemoryError: Java heap space
# -XX:OnOutOfMemoryError="/bin/echo hello"
#   Executing "/bin/echo hello"...
hello
ResourceExhausted: killing current process!
make: *** [test] Killed: 9
```