# Makefile for Zerofetch
# Installation script for Linux systems

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
MANDIR = $(PREFIX)/share/man/man1
CONFDIR = $(HOME)/.config/zerofetch

.PHONY: all install uninstall clean

all:
	@echo "Run 'make install' to install Zerofetch"
	@echo "Run 'make uninstall' to uninstall Zerofetch"

install:
	@echo "Installing Zerofetch..."
	@echo "Installing required dependencies..."
	@if command -v pacman >/dev/null 2>&1; then \
		sudo pacman -S --noconfirm --needed pciutils xorg-xrandr lm_sensors playerctl 2>/dev/null || true; \
	elif command -v apt >/dev/null 2>&1; then \
		sudo apt install -y pciutils x11-xserver-utils lm-sensors playerctl 2>/dev/null || true; \
	elif command -v dnf >/dev/null 2>&1; then \
		sudo dnf install -y pciutils xrandr lm_sensors playerctl 2>/dev/null || true; \
	elif command -v zypper >/dev/null 2>&1; then \
		sudo zypper install -y pciutils xrandr sensors playerctl 2>/dev/null || true; \
	fi
	@mkdir -p $(BINDIR)
	@mkdir -p $(MANDIR)
	@mkdir -p $(CONFDIR)
	@install -m 755 zerofetch $(BINDIR)/zerofetch
	@if [ -f zerofetch.1 ]; then \
		install -m 644 zerofetch.1 $(MANDIR)/zerofetch.1; \
		gzip -f $(MANDIR)/zerofetch.1; \
	fi
	@if [ -f zerofetch.conf ]; then \
		if [ ! -f $(CONFDIR)/config ]; then \
			install -m 644 zerofetch.conf $(CONFDIR)/config; \
		fi; \
	fi
	@echo "Zerofetch installed successfully!"
	@echo "Run 'zerofetch' to use it"

uninstall:
	@echo "Uninstalling Zerofetch..."
	@rm -f $(BINDIR)/zerofetch
	@rm -f $(MANDIR)/zerofetch.1.gz
	@echo "Zerofetch uninstalled successfully!"
	@echo "Note: Config directory $(CONFDIR) was not removed"

clean:
	@echo "Nothing to clean"
