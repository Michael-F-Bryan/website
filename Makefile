BINARY := website-server
TARGET := release
TARGET_DIR := $(shell pwd)/target
RELEASE_BINARY := $(TARGET_DIR)/$(TARGET)/$(BINARY)
ASSETS := $(wildcard templates/** static/**)
ROCKET_TOML := Rocket.toml

TEMP_DIR := $(TARGET_DIR)/package
TEMP_DIR_BINARY := $(TEMP_DIR)/$(BINARY)
TEMP_DIR_ASSETS := $(patsubst %,$(TEMP_DIR)/%,$(ASSETS))
TEMP_DIR_ROCKET_TOML := $(TEMP_DIR)/$(ROCKET_TOML)

PACKAGED := $(TARGET_DIR)/packaged.zip

help:
	@echo "package              Compile the application and bundle all assets"
	@echo "                       into a single zip file (target/packaged.zip)"
	@echo "help                 Print this help message"

package: $(PACKAGED)

$(PACKAGED): $(TEMP_DIR_BINARY) $(TEMP_DIR_ASSETS) $(TEMP_DIR_ROCKET_TOML)
	cd $(TEMP_DIR) && zip -r -9 -u $(PACKAGED) *

$(TEMP_DIR_BINARY): $(TEMP_DIR) $(RELEASE_BINARY)
	cp $(RELEASE_BINARY) $(TEMP_DIR_BINARY)

$(TEMP_DIR):
	mkdir -p $(TEMP_DIR)

$(TEMP_DIR)/templates/%: templates/%
	cp -r templates $(TEMP_DIR)

$(TEMP_DIR)/static/%: static/%
	cp -r static $(TEMP_DIR)

$(RELEASE_BINARY):
	cargo build --release

$(TEMP_DIR_ROCKET_TOML): $(ROCKET_TOML)
	cp $(ROCKET_TOML) $(TEMP_DIR_ROCKET_TOML)

clean:
	rm -r $(TEMP_DIR)