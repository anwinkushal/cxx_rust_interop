// src/blobstore.cc

#include "rust_cxx_class/include/E1_addition.h"

Addition_Class::Addition_Class() {}

uint64_t Addition_Class::add(const uint64_t &a,const uint64_t &b) const{

  return a+b;
}

uint64_t Addition_Class::add_vector(const rust::Vec<uint32_t> &input_vec) const
{
  std::vector<uint32_t> vec(input_vec.begin(), input_vec.end());
  uint64_t addition = 0u;
  for (auto v: vec)
  {
    addition += v;
  }
    return addition;
}
std::unique_ptr<Addition_Class> new_Addition_Class()
{
    return std::unique_ptr<Addition_Class>(new Addition_Class());
}