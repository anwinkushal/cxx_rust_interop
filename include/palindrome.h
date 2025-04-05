
#pragma once
#include <memory>
#include <vector>

class Palindrome_Class
{
public:
    Palindrome_Class();
    bool is_palindrome(const int32_t x) const;
};

std::unique_ptr<Palindrome_Class> new_Palindrome_Class();