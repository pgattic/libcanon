
NAME=canon
PKG_NAME=libcanon
PREFIX=/usr/local

all:
	cargo build --release

install:
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f target/release/${PKG_NAME} ${DESTDIR}${PREFIX}/bin/${NAME}
	chmod +x ${DESTDIR}${PREFIX}/bin/${NAME}

.PHONY: all install


