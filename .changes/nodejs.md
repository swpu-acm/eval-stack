---
"eval-stack": minor:feat
---

Add support for Deno.js, and other changes:

- Use deno instead of node for executing JavaScript files, deny all permissions by default.
- Allow stderr to be piped since we can capture it for error messages.
- Disable core dumps by default.
- Set CPU time limits using `libc`.
- Use seccomp to restrict sys calls and fs operations.
- Prevent process to create subprocesses.
