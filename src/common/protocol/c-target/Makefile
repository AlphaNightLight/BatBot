#DEST = ../c-target

SRC_FOLDER = ../c-src
CPPFILES=$(foreach D,$(SRC_FOLDER),$(wildcard $(D)/*.cpp))
TARGETS=$(patsubst $(SRC_FOLDER)/%.cpp,%.o,$(CPPFILES))
DEPS=$(patsubst $(SRC_FOLDER)/%.cpp,%.d,$(CPPFILES))
#DEP= -Ichecker.hpp -Iprotocol.hpp -Iconst.hpp
fresh: clear all

CPPFLAGS += -MMD -MP

all: libc-protocol.a

libc-protocol.a: $(TARGETS)
	ar rcs libc-protocol.a $(TARGETS)

%.o : $(SRC_FOLDER)/%.cpp
	clang -MD -MP -Wall -O3 -o $@ -c $(SRC_FOLDER)/$*.cpp
#$(SRC_FOLDER)/$*.cpp
#$^ 
#$(DEP)

%.d: $(SRC_FOLDER)/%.cpp
	touch $@

make.d: $(DEPS)
	-cat $(DEPS) > $@

#%.cpp: %.hpp
-include make.d

clear:
	rm -rf *.o *.a *.d
