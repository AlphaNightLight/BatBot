DEST = ../c-build
SRC_FOLDER = .
CPPFILES=$(foreach D,$(SRC_FOLDER),$(wildcard $(D)/*.cpp))
TARGETS=$(patsubst %.cpp,%.o,$(CPPFILES))
DEP= -Ichecker.hpp -Iprotocol.hpp -Iconst.hpp


all: $(TARGETS)
	ar rcs ./libc-protocol.a $(TARGETS)

%.o : %.cpp
	clang -Wall -c $(DEP) -O3 -o $@ $^

%.cpp: %.hpp
	

clear:
	rm -r *.o
	rm -r *.a