SHELL = /bin/sh

# clear, then define suffix list
.SUFFIXES:
.SUFFIXES: .puml .svg .png

SRC = pdm.puml database/erd.puml
PNG_TARGS = $(patsubst %.puml,%.png,$(SRC))
SVG_TARGS = $(patsubst %.puml,%.svg,$(SRC))

PLANTUML_FLAGS = -nbthread auto

.PHONY: all png svg
all: png svg

png: $(PNG_TARGS)

svg: $(SVG_TARGS)

%.png: %.puml
    # Note: There seems to be no option for setting regular output file.
	cat $< | plantuml $(PLANTUML_FLAGS) -pipe -tpng > $@

%.svg: %.puml
	cat $< | plantuml $(PLANTUML_FLAGS) -pipe -tsvg > $@

.PHONY: clean
clean :
	# delete all regular files ending in {~,o}
	-@find . -name "*~" -type f -print0 | xargs -0 /bin/rm -f
	-@rm $(PNG_TARGS) $(SVG_TARGS)