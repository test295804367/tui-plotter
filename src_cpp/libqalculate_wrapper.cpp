
#include "libqalculate_wrapper.hpp"

extern "C" {
    Calculator* calculator_new() {
        return new Calculator();
    }

    const char* calculator_evaluate(Calculator* calc, const char* expression) {
        static std::string result;
        result = calc->calculate(expression).print();
        return result.c_str();
    }

    void calculator_free(Calculator* calc) {
        delete calc;
    }
}
