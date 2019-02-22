PG_CONFIG ?= pg_config
CARGO := cargo

PG_INCLUDE_DIR = $(shell $(PG_CONFIG) --includedir-server)
PG_INSTALL_LIB_DIR = $(shell $(PG_CONFIG) --pkglibdir)
PG_INSTALL_SHARE_DIR = $(shell $(PG_CONFIG) --sharedir)/extension


# https://stackoverflow.com/questions/714100/os-detecting-makefile
ifeq ($(OS),Windows_NT)
	SHARED_LIB_EXTENSION=dll
else
  UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Darwin)
		SHARED_LIB_EXTENSION=dylib
	endif
	ifeq ($(UNAME_S),Linux)
		SHARED_LIB_EXTENSION=so
	endif
endif

TARGET := "target/release/libpokemon_fdw.$(SHARED_LIB_EXTENSION)"

.PHONY: all build install

all: build

build:
	PG_INCLUDE_PATH=$(PG_INCLUDE_DIR) cargo build --release

install:
	install $(TARGET) $(PG_INSTALL_LIB_DIR)
	install -m 644 pokemon.control $(PG_INSTALL_SHARE_DIR)
	install -m 644 pokemon--0.1.0.sql $(PG_INSTALL_SHARE_DIR)
