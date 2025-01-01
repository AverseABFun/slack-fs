# SPDX-License-Identifier: GPL-2.0
.PHONY: default modules_install

KDIR ?= kernel-build

default:
	$(MAKE) -C $(KDIR) M=$$PWD LLVM=1

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD LLVM=1 modules_install

rust-analyzer:
	$(MAKE) -C $(KDIR) M=$$PWD LLVM=1 rust-analyzer