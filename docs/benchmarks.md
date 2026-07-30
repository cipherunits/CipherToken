---
title: Benchmarks — Fastest Python JWT Library
description: CipherToken benchmark results vs PyJWT and python-jose. Up to 5.8x faster token creation and 8.9x faster verification with the HS256 algorithm.
keywords: python jwt benchmark, fastest jwt library python, pyjwt performance, jwt speed comparison, ciphertoken benchmark
image: https://cipherunits.github.io/CipherToken/logo.png
---

# CipherToken Benchmarks

CipherToken is a high-performance Python token library designed for applications that require fast token creation and verification with minimal overhead.

This benchmark compares CipherToken against popular Python JWT libraries using the HS256 algorithm.

---

## Benchmark Summary

🏆 **CipherToken achieved the highest performance in every benchmark category tested.**

| Library | Token Creation (ops/sec) | Token Verification (ops/sec) |
|----------|----------:|----------:|
| **CipherToken** | **587,156** | **244,500** |
| PyJWT | 101,591 | 35,928 |
| python-jose | 103,861 | 27,638 |

---

## Performance Advantage

### Token Creation

- CipherToken is **5.78x faster** than PyJWT.
- CipherToken is **5.65x faster** than python-jose.

### Token Verification

- CipherToken is **6.80x faster** than PyJWT.
- CipherToken is **8.85x faster** than python-jose.

---

## Benchmark Environment

- Python 3.14
- HS256 algorithm
- 100,000 iterations per benchmark
- Average of 5 runs
- Identical payload for all libraries

### Payload

```python
{
    "user_id": 123,
    "username": "ehsan",
    "role": "admin"
}
```

---

## Why CipherToken Is Faster

CipherToken is designed with performance as a primary goal. The entire signing and verification pipeline runs in compiled **Rust** via PyO3, so Python only pays the cost of a single native call.

Key optimizations include:

- Minimal runtime overhead
- Efficient token generation
- Optimized token verification
- Modern implementation focused on high throughput
- Suitable for APIs, authentication services, and microservices

---

## Real-World Benefits

Higher throughput means:

- Faster API responses
- Lower CPU usage
- Better scalability
- More requests handled per server
- Reduced infrastructure costs

---

## Benchmark Notes

The benchmark was executed on the same machine under identical conditions.

Results may vary depending on:

- Hardware
- Python version
- Payload size
- Selected algorithm
- Application workload

However, the benchmark consistently showed CipherToken outperforming the compared libraries in both token creation and verification workloads.

---

## Conclusion

For applications requiring fast token operations, CipherToken demonstrated the strongest performance in this benchmark, achieving significantly higher throughput than PyJWT and python-jose while maintaining a simple developer experience.

Next steps:

- [Install CipherToken](getting-started/installation.md)
- [Quick Start](getting-started/quick-start.md)
- [Feature comparison with PyJWT and python-jose](comparison.md)
