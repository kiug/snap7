# Build and run the Snap7 C++ unit tests.
#
#   make            build the test binary
#   make test       build and run
#   make cov        build with --coverage, run, and generate gcov reports
#   make cov-build  build the coverage-instrumented binary (no run)
#   make cov-run    run the already-built coverage binary
#   make cov-report generate gcov reports from an already-run coverage build
#   make clean      remove build artifacts

CXX      ?= g++
CXXFLAGS ?= -O0 -g -std=c++14 -Wall
INCLUDES := -I../src/sys -I../src/core -I../src/lib
LIBS     := -lpthread -lrt
BUILD    := build

SNAP7_SOURCES := \
	../src/sys/snap_msgsock.cpp \
	../src/sys/snap_sysutils.cpp \
	../src/sys/snap_tcpsrvr.cpp \
	../src/sys/snap_threads.cpp \
	../src/core/s7_client.cpp \
	../src/core/s7_isotcp.cpp \
	../src/core/s7_partner.cpp \
	../src/core/s7_peer.cpp \
	../src/core/s7_server.cpp \
	../src/core/s7_text.cpp \
	../src/core/s7_micro_client.cpp \
	../src/lib/snap7_libmain.cpp

TEST_SOURCES := \
	test_main.cpp \
	test_text.cpp \
	test_client.cpp \
	test_server.cpp \
	test_integration.cpp \
	test_partner.cpp \
	test_whitebox.cpp \
	test_socket.cpp

SNAP7_OBJS := $(SNAP7_SOURCES:../%.cpp=$(BUILD)/%.o)
TEST_OBJS  := $(TEST_SOURCES:%.cpp=$(BUILD)/%.o)
DEPS       := $(SNAP7_OBJS:.o=.d) $(TEST_OBJS:.o=.d)

TARGET := $(BUILD)/snap7_tests

.PHONY: all test cov cov-build cov-run cov-report cov-check cov-html clean setcap

all: $(TARGET)

# Compile with absolute source paths so that gcov can resolve them from any
# working directory. -MMD -MP tracks header dependencies so that editing a
# Snap7 header correctly rebuilds every translation unit that includes it.
$(BUILD)/%.o: ../%.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(COVERAGE_FLAGS) $(INCLUDES) -MMD -MP -c $(abspath $<) -o $@

$(BUILD)/%.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(COVERAGE_FLAGS) $(INCLUDES) -MMD -MP -c $(abspath $<) -o $@

# test_whitebox.cpp reaches private/protected members; compiling it with
# -fno-access-control is cleaner than the `#define private public` macro hack
# and keeps the access specifiers untouched for every other translation unit.
$(BUILD)/test_whitebox.o: test_whitebox.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(COVERAGE_FLAGS) $(INCLUDES) -fno-access-control -MMD -MP -c $(abspath $<) -o $@

$(TARGET): $(SNAP7_OBJS) $(TEST_OBJS)
	$(CXX) $(CXXFLAGS) $(COVERAGE_FLAGS) -o $@ $^ $(LIBS)

test: $(TARGET)
	./$(TARGET)

# Grant the test binary permission to bind the privileged port 102. This must
# be re-applied after every rebuild (make/make cov/make clean remove it).
setcap: $(TARGET)
	sudo setcap 'cap_net_bind_service=+ep' $(TARGET)

# Build with coverage instrumentation, without running.
cov-build:
	$(MAKE) COVERAGE_FLAGS="--coverage" $(TARGET)

# Run the already-built coverage binary. Apply setcap between cov-build and
# cov-run so the partner round-trip (privileged port 102) is exercised.
cov-run:
	./$(TARGET)

cov: clean
	$(MAKE) cov-build
	./$(TARGET)
	$(MAKE) cov-report

cov-report:
	@echo "Generating gcov reports..."
	@for d in $(BUILD) $(BUILD)/src/sys $(BUILD)/src/core $(BUILD)/src/lib; do \
		if [ -d "$$d" ]; then \
			(cd "$$d" && for obj in *.o; do \
				gcov -b -c "$$obj" > /dev/null 2>&1 || true; \
			done); \
		fi; \
	done
	@echo ""
	@echo "Per-file reports are in $(BUILD)/**/*.gcov"
	@echo ""
	@echo "================ C++ COVERAGE (gcov) ================"
	@total=0; covered=0; \
	for f in $$(find $(BUILD) -name '*.cpp.gcov' ! -name 'test_*.cpp.gcov'); do \
		vals=$$(awk -F: ' \
			{ n=$$1; gsub(/^[ \t]+|[ \t]+$$/, "", n); \
			  if (n ~ /^[0-9]+$$/) { t++; if (n+0 > 0) c++; } \
			  else if (n == "#####") { t++; } } \
			END { printf "%d %d", c+0, t+0 }' "$$f"); \
		c=$${vals% *}; t=$${vals#* }; \
		covered=$$((covered + c)); total=$$((total + t)); \
	done; \
	echo "  covered lines: $$covered"; \
	echo "  total lines:   $$total"; \
	if [ "$$total" -gt 0 ]; then \
		awk -v c="$$covered" -v t="$$total" 'BEGIN{printf "  line coverage: %.2f%%\n", 100*c/t}'; \
	fi; \
	echo "====================================================="

# Enforce a line-coverage threshold with gcovr. Run after cov/cov-build+cov-run
# so the .gcda data is present. Requires: pip install gcovr.
cov-check:
	gcovr --root "$(abspath ..)" --object-directory "$(abspath $(BUILD))" \
		--filter 'src/.*\.cpp$$' \
		--fail-under-line 85 --print-summary

clean:
	rm -rf $(BUILD)

-include $(DEPS)

# HTML + branch-coverage report (requires lcov + genhtml).
cov-html: clean
	$(MAKE) COVERAGE_FLAGS="--coverage" $(TARGET)
	./$(TARGET)
	@if command -v lcov >/dev/null 2>&1; then \
		lcov --capture --directory $(BUILD) --output-file $(BUILD)/coverage.info \
			--rc lcov_branch_coverage=1 --quiet; \
		lcov --remove $(BUILD)/coverage.info '/usr/*' '*/test_*.cpp' '*/test.h' \
			--output-file $(BUILD)/coverage.info --rc lcov_branch_coverage=1 --quiet; \
		genhtml $(BUILD)/coverage.info --branch-coverage \
			--output-directory $(BUILD)/html --quiet; \
		echo "HTML report: $(BUILD)/html/index.html"; \
	else \
		echo "lcov/genhtml not installed. Install with: sudo apt install lcov"; \
	fi
