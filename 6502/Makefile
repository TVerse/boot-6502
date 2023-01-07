TOOLS_BASE=~/coding/cc65/bin
CA65_BINARY=$(TOOLS_BASE)/ca65
LD65_BINARY=$(TOOLS_BASE)/ld65
ROM_NAME=dummy

CPU_FLAG=--cpu 65c02

CA65_FLAGS=$(CPU_FLAG)

BUILD_FOLDER=./build

TEMP_FOLDER=$(BUILD_FOLDER)/$(ROM_NAME)
ROM_FILE=$(BUILD_FOLDER)/$(ROM_NAME).bin
MAP_FILE=$(TEMP_FOLDER)/$(ROM_NAME).map

ASM_OBJECTS=$(ASM_SOURCES:%.s=$(TEMP_FOLDER)/%.o)

ASM_SOURCES=memory_map.s memory.s debug.s via.s acia.s lcd.s base.s dummy.s

FIRMWARE_CFG=firmware.cfg

$(TEMP_FOLDER)/%.o: %.s
	@mkdir -p $(TEMP_FOLDER)
	$(CA65_BINARY) $(CA65_FLAGS) -o $@ -l $(@:.o=.lst) $<

$(ROM_FILE): $(ASM_OBJECTS) $(FIRMWARE_CFG)
	@mkdir -p $(BUILD_FOLDER)
	$(LD65_BINARY) -C $(FIRMWARE_CFG) -o $@ -m $(MAP_FILE) $(ASM_OBJECTS)

all: $(ROM_FILE)

flash: $(ROM_FILE)
	$(MINIPRO_BINARY) TODO

clean:
	rm -f $(ROM_FILE) $(MAP_FILE) $(ASM_OBJECTS) $(ASM_OBJECTS:%.o=%.lst)


.PHONY: list
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$'
