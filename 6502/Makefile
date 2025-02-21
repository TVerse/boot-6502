AS := ca65
LD := ld65

export TOP_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
export BIN_DIR := $(TOP_DIR)/bin
export BUILD_DIR := $(TOP_DIR)/build
export CONF_DIR := $(TOP_DIR)/conf
export SRC_DIR := $(TOP_DIR)/src
export INC_DIR := $(TOP_DIR)/include

LD_CONF := $(CONF_DIR)/ld.cfg

SRCS := $(wildcard $(SRC_DIR)/*.s)
OBJS := $(SRCS:$(SRC_DIR)/%.s=$(BUILD_DIR)/%.o)
INCS := $(wildcard $(INC_DIR)/*.inc)

NAME := boot-6502
ROM := $(BIN_DIR)/$(NAME).img
DBG := $(ROM:%.img=%.dbg)

AS_FLAGS := --cpu 65c02
AS_FLAGS += --feature underline_in_numbers

LD_FLAGS := -C $(LD_CONF) --dbgfile $(DBG)

.PHONY: all
all: $(ROM)

.PHONY: clean
clean:
	-rm -rf $(BUILD_DIR)
	-rm -rf $(BIN_DIR)

$(ROM): $(OBJS) $(LD_CONF) $(BIN_DIR)
	$(LD) $(LD_FLAGS) -o $@ $(OBJS)

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s $(INCS) $(BUILD_DIR)
	$(AS) $(AS_FLAGS) -o $@ $<

$(BUILD_DIR):
	-mkdir $@

$(BIN_DIR):
	-mkdir $@
