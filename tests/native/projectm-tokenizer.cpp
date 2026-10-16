#include <cstddef>
#include <cstring>
#include <string>
#include <iostream>
#include "HLSLTokenizer.h"
int main() {
  for (size_t n : {255u, 256u, 4096u}) {
    std::string source(n, 'a');
    M4::HLSLTokenizer tokenizer("boundary.hlsl", source.data(), source.size());
    if (n == 255) {
      if (tokenizer.GetToken() != M4::HLSLToken_Identifier || std::strlen(tokenizer.GetIdentifier()) != n) return 1;
      tokenizer.Next();
    }
    if (tokenizer.GetToken() != M4::HLSLToken_EndOfStream) return 2;
  }
  const std::string shader = "float amount = 1;";
  M4::HLSLTokenizer normal("normal.hlsl", shader.data(), shader.size());
  for (int expected : {int(M4::HLSLToken_Float), int(M4::HLSLToken_Identifier), int('='), int(M4::HLSLToken_IntLiteral), int(';'), int(M4::HLSLToken_EndOfStream)}) {
    if (normal.GetToken() != expected) return 3;
    normal.Next();
  }
  std::cout << "PASS: 255 accepted; 256 and 4096 rejected; normal token sequence preserved\n";
}
