---
tags: computer science
crystal-type: entity
crystal-domain: computer science
---
Software layer managing hardware resources and providing an environment for programs to run. The bridge between silicon and [[algorithms]].

## core responsibilities

process scheduling: allocating CPU time among competing tasks, context switching, priorities

memory management: virtual memory, paging, allocation, protection between processes

file systems: organizing persistent storage into hierarchical namespaces

device drivers: abstracting hardware interfaces for uniform access

security: user permissions, access control, isolation

## types

monolithic kernel: Linux, all services in kernel space

microkernel: Minix, minimal kernel with services in user space

exokernel: application-level resource management

unikernel: single-application, library OS

## key abstractions

The process, the file, and the socket. Everything in Unix is a file. Processes communicate through pipes, shared memory, and message passing. Sockets extend communication across [[networks]].

## relation to the stack

[[compilers]] produce binaries that the OS loads and schedules. [[databases]] rely on OS file systems and memory management. [[consensus algorithms]] run as distributed OS-level processes.