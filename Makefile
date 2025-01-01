# SPDX-License-Identifier: GPL-2.0
.PHONY: default modules_install

KDIR ?= kernel-build
LLVM ?= 0

default:
	$(MAKE) -C $(KDIR) M=$$PWD LLVM=$(LLVM)

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD LLVM=$(LLVM) modules_install