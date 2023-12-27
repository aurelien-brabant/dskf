TARGET		:= target/release/dskf
RM			:= rm -rf
INSTALL_DIR	:= /usr/local/bin

all: $(TARGET)

$(TARGET):
	cargo build --release
.PHONY: $(TARGET)

install:
	pkexec --user root sh -c "cp $(PWD)/$(TARGET) $(INSTALL_DIR)"

clean:
	$(RM) $(TARGET)

fclean:
	$(RM) target

re: fclean all
