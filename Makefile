ifndef JAVA_HOME
    $(error JAVA_HOME not set)
endif

ifeq ($(shell uname -s),Darwin)
  INCLUDE= -I"$(JAVA_HOME)/include" -I"$(JAVA_HOME)/include/darwin"
else
  INCLUDE= -I"$(JAVA_HOME)/include" -I"$(JAVA_HOME)/include/linux"
endif

CFLAGS=-Wall -Werror -fPIC -shared $(INCLUDE)
TARGET=libjvmkill.so

.PHONY: all clean test test2

all:
	gcc $(CFLAGS) \
		-o $(TARGET) \
		jvmkill.c
	chmod 644 $(TARGET)

clean:
	rm -f $(TARGET)
	rm -f *.class
	rm -f *.hprof

test: all
	$(JAVA_HOME)/bin/javac JvmKillTest.java
	$(JAVA_HOME)/bin/java -Xmx5m \
	    -XX:+HeapDumpOnOutOfMemoryError \
	    -XX:OnOutOfMemoryError='/bin/echo hello' \
	    -agentpath:$(PWD)/$(TARGET) \
	    -cp $(PWD) JvmKillTest

test2: all
	$(JAVA_HOME)/bin/javac JvmKillTest2.java
	ulimit -u 
	$(JAVA_HOME)/bin/java \
	    -agentpath:$(PWD)/$(TARGET) \
	    -cp $(PWD) JvmKillTest2
