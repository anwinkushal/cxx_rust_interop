#include "rust_cxx_class/include/palindrome.h"

Palindrome_Class::Palindrome_Class() {}

bool Palindrome_Class::is_palindrome(const int32_t value) const
{
    int32_t x = value;
    if (x < 0 || (x != 0 && x % 10 == 0))
    {
        return false;
    }

    int reversed = 0;
    while (x > reversed)
    {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }
    return (x == reversed) || (x == reversed / 10);
}

std::unique_ptr<Palindrome_Class> new_Palindrome_Class()
{
    return std::unique_ptr<Palindrome_Class>(new Palindrome_Class());
}