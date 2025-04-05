
#pragma once
#include <memory>
#include <vector>
#include "rust/cxx.h"

struct AdderInput;

class Addition_Class
{
public:
    Addition_Class();
    uint64_t add(const uint64_t &a, const uint64_t &b) const;
    uint64_t add_vector(const rust::Vec<uint32_t> &input_vec) const;

private:
    uint8_t test = 1U;
};

std::unique_ptr<Addition_Class> new_Addition_Class();