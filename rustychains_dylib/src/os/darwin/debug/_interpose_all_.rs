// pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t) -> ::ssize_t;
// pub fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t) -> ::ssize_t;
// pub fn socket(domain: ::c_int, ty: ::c_int, protocol: ::c_int) -> ::c_int;
// pub fn connect(socket: ::c_int, address: *const sockaddr, len: socklen_t) -> ::c_int;
// pub fn close(fd: ::c_int) -> ::c_int;

// pub fn usleep(secs: ::c_uint) -> ::c_int;
// pub fn malloc(size: size_t) -> *mut c_void;
// pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
// pub fn free(p: *mut c_void);
// pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
// pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
// pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
// pub fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;

// pub fn strdup(cs: *const c_char) -> *mut c_char;

// pub fn snprintf(s: *mut ::c_char, n: ::size_t, format: *const ::c_char, ...) -> ::c_int;

// pub fn pthread_setspecific(key: pthread_key_t, value: *const ::c_void) -> ::c_int;


// WORKING:

// pub fn pthread_cond_wait(cond: *mut pthread_cond_t, lock: *mut pthread_mutex_t) -> ::c_int;
// pub fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> ::c_int;
// pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> ::c_int;
// pub fn pthread_mutex_init(lock: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> ::c_int;

// pub fn pthread_self() -> ::pthread_t;
// pub fn pthread_join(native: ::pthread_t, value: *mut *mut ::c_void) -> ::c_int;
// pub fn pthread_exit(value: *mut ::c_void) -> !;
// pub fn pthread_attr_init(attr: *mut ::pthread_attr_t) -> ::c_int;
// pub fn pthread_attr_destroy(attr: *mut ::pthread_attr_t) -> ::c_int;
// pub fn pthread_attr_setstacksize(attr: *mut ::pthread_attr_t, stack_size: ::size_t) -> ::c_int;
// pub fn pthread_attr_setdetachstate(attr: *mut ::pthread_attr_t, state: ::c_int) -> ::c_int;
// pub fn pthread_detach(thread: ::pthread_t) -> ::c_int;
// pub fn sched_yield() -> ::c_int;
// pub fn pthread_key_create(key: *mut pthread_key_t, dtor: ::Option<unsafe extern "C" fn(*mut ::c_void)>) -> ::c_int;
// pub fn pthread_key_delete(key: pthread_key_t) -> ::c_int;
// pub fn pthread_getspecific(key: pthread_key_t) -> *mut ::c_void;

// pub fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> ::c_int;
// pub fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> ::c_int;
// pub fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> ::c_int;
// pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> ::c_int;
// pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> ::c_int;
// pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, _type: ::c_int) -> ::c_int;

// pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t, lock: *mut pthread_mutex_t, abstime: *const ::timespec) -> ::c_int;
// pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> ::c_int;
// pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> ::c_int;
// pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> ::c_int;
// pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> ::c_int;
// pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> ::c_int;
// pub fn pthread_rwlock_init(lock: *mut pthread_rwlock_t, attr: *const pthread_rwlockattr_t) -> ::c_int;
// pub fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlock_tryrdlock(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlock_trywrlock(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> ::c_int;
// pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> ::c_int;
// pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> ::c_int;

// pub fn isalnum(c: c_int) -> c_int;
// pub fn isalpha(c: c_int) -> c_int;
// pub fn iscntrl(c: c_int) -> c_int;
// pub fn isdigit(c: c_int) -> c_int;
// pub fn isgraph(c: c_int) -> c_int;
// pub fn islower(c: c_int) -> c_int;
// pub fn isprint(c: c_int) -> c_int;
// pub fn ispunct(c: c_int) -> c_int;
// pub fn isspace(c: c_int) -> c_int;
// pub fn isupper(c: c_int) -> c_int;
// pub fn isxdigit(c: c_int) -> c_int;
// pub fn isblank(c: c_int) -> c_int;
// pub fn tolower(c: c_int) -> c_int;
// pub fn toupper(c: c_int) -> c_int;
// pub fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: ::Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
// pub fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: ::Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void;
// pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
// pub fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE;
// pub fn fflush(file: *mut FILE) -> c_int;
// pub fn fclose(file: *mut FILE) -> c_int;
// pub fn remove(filename: *const c_char) -> c_int;
// pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
// pub fn tmpfile() -> *mut FILE;
// pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int;
// pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
// pub fn getchar() -> c_int;
// pub fn putchar(c: c_int) -> c_int;
// pub fn fgetc(stream: *mut FILE) -> c_int;
// pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
// pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
// pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
// pub fn puts(s: *const c_char) -> c_int;
// pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
// pub fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
// pub fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
// pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
// pub fn ftell(stream: *mut FILE) -> c_long;
// pub fn rewind(stream: *mut FILE);
// pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
// pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
// pub fn feof(stream: *mut FILE) -> c_int;
// pub fn ferror(stream: *mut FILE) -> c_int;
// pub fn clearerr(stream: *mut FILE);
// pub fn perror(s: *const c_char);
// pub fn atof(s: *const c_char) -> c_double;
// pub fn atoi(s: *const c_char) -> c_int;
// pub fn atol(s: *const c_char) -> c_long;
// pub fn atoll(s: *const c_char) -> c_longlong;
// pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
// pub fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float;
// pub fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long;
// pub fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong;
// pub fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;
// pub fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong;
// pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;

// pub fn abort() -> !;
// pub fn exit(status: c_int) -> !;
// pub fn _exit(status: c_int) -> !;
// pub fn system(s: *const c_char) -> c_int;
// pub fn getenv(s: *const c_char) -> *mut c_char;

// pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
// pub fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
// pub fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
// pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
// pub fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char;
// // pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
// pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
// pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
// // pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
// pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;

// pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
// pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
// pub fn strndup(cs: *const c_char, n: size_t) -> *mut c_char;
// pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
// pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
// pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
// pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
// // pub fn strlen(cs: *const c_char) -> size_t;
// pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
// pub fn strerror(n: c_int) -> *mut c_char;
// pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
// pub fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char;
// pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
// pub fn strsignal(sig: c_int) -> *mut c_char;
// pub fn wcslen(buf: *const wchar_t) -> size_t;
// pub fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> ::size_t;

// pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
// pub fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;

// pub fn getpwnam(name: *const ::c_char) -> *mut passwd;
// pub fn getpwuid(uid: ::uid_t) -> *mut passwd;

// pub fn fprintf(stream: *mut ::FILE, format: *const ::c_char, ...) -> ::c_int;
// pub fn printf(format: *const ::c_char, ...) -> ::c_int;

// pub fn sprintf(s: *mut ::c_char, format: *const ::c_char, ...) -> ::c_int;
// pub fn fscanf(stream: *mut ::FILE, format: *const ::c_char, ...) -> ::c_int;
// pub fn scanf(format: *const ::c_char, ...) -> ::c_int;
// pub fn sscanf(s: *const ::c_char, format: *const ::c_char, ...) -> ::c_int;
// pub fn getchar_unlocked() -> ::c_int;
// pub fn putchar_unlocked(c: ::c_int) -> ::c_int;


pub fn listen(socket: ::c_int, backlog: ::c_int) -> ::c_int;
pub fn accept(socket: ::c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> ::c_int;
// pub fn getpeername(socket: ::c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> ::c_int;
pub fn getsockname(socket: ::c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> ::c_int;
pub fn setsockopt(socket: ::c_int, level: ::c_int, name: ::c_int, value: *const ::c_void, option_len: socklen_t) -> ::c_int;
pub fn socketpair(domain: ::c_int, type_: ::c_int, protocol: ::c_int, socket_vector: *mut ::c_int) -> ::c_int;
pub fn sendto(socket: ::c_int, buf: *const ::c_void, len: ::size_t, flags: ::c_int, addr: *const sockaddr, addrlen: socklen_t) -> ::ssize_t;
pub fn shutdown(socket: ::c_int, how: ::c_int) -> ::c_int;

// pub fn chmod(path: *const c_char, mode: mode_t) -> ::c_int;
// pub fn fchmod(fd: ::c_int, mode: mode_t) -> ::c_int;

// pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;

// pub fn mkdir(path: *const c_char, mode: mode_t) -> ::c_int;

// pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;

// pub fn pclose(stream: *mut ::FILE) -> ::c_int;
// pub fn fdopen(fd: ::c_int, mode: *const c_char) -> *mut ::FILE;
// pub fn fileno(stream: *mut ::FILE) -> ::c_int;

// pub fn open(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
// pub fn creat(path: *const c_char, mode: mode_t) -> ::c_int;
// pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;

// pub fn opendir(dirname: *const c_char) -> *mut ::DIR;

// pub fn readdir(dirp: *mut ::DIR) -> *mut ::dirent;
// pub fn closedir(dirp: *mut ::DIR) -> ::c_int;
// pub fn rewinddir(dirp: *mut ::DIR);

// pub fn fchmodat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t, flags: ::c_int) -> ::c_int;
// pub fn fchown(fd: ::c_int, owner: ::uid_t, group: ::gid_t) -> ::c_int;
// pub fn fchownat(dirfd: ::c_int, pathname: *const ::c_char, owner: ::uid_t, group: ::gid_t, flags: ::c_int) -> ::c_int;
// pub fn fstatat(dirfd: ::c_int, pathname: *const ::c_char, buf: *mut stat, flags: ::c_int) -> ::c_int;
// pub fn linkat(olddirfd: ::c_int, oldpath: *const ::c_char, newdirfd: ::c_int, newpath: *const ::c_char, flags: ::c_int) -> ::c_int;
// pub fn renameat(olddirfd: ::c_int, oldpath: *const ::c_char, newdirfd: ::c_int, newpath: *const ::c_char) -> ::c_int;
// pub fn symlinkat(target: *const ::c_char, newdirfd: ::c_int, linkpath: *const ::c_char) -> ::c_int;
// pub fn unlinkat(dirfd: ::c_int, pathname: *const ::c_char, flags: ::c_int) -> ::c_int;

// pub fn access(path: *const c_char, amode: ::c_int) -> ::c_int;
// pub fn alarm(seconds: ::c_uint) -> ::c_uint;
// pub fn chdir(dir: *const c_char) -> ::c_int;
// pub fn fchdir(dirfd: ::c_int) -> ::c_int;
// pub fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> ::c_int;
// pub fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> ::c_int;
// pub fn dup(fd: ::c_int) -> ::c_int;
// pub fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int;
// pub fn execl(path: *const c_char, arg0: *const c_char, ...) -> ::c_int;
// pub fn execle(path: *const ::c_char, arg0: *const ::c_char, ...) -> ::c_int;
// pub fn execlp(file: *const ::c_char, arg0: *const ::c_char, ...) -> ::c_int;
// pub fn execv(prog: *const c_char, argv: *const *const c_char) -> ::c_int;
// pub fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> ::c_int;
// pub fn execvp(c: *const c_char, argv: *const *const c_char) -> ::c_int;
// pub fn fork() -> pid_t;
// pub fn fpathconf(filedes: ::c_int, name: ::c_int) -> c_long;
// pub fn getcwd(buf: *mut c_char, size: ::size_t) -> *mut c_char;
// pub fn getegid() -> gid_t;
// pub fn geteuid() -> uid_t;
// pub fn getgid() -> gid_t;
// pub fn getgroups(ngroups_max: ::c_int, groups: *mut gid_t) -> ::c_int;
// pub fn getlogin() -> *mut c_char;
// pub fn getopt(argc: ::c_int, argv: *const *mut c_char, optstr: *const c_char) -> ::c_int;
// pub fn getpgid(pid: pid_t) -> pid_t;
// pub fn getpgrp() -> pid_t;
// pub fn getpid() -> pid_t;
// pub fn getppid() -> pid_t;
// pub fn getuid() -> uid_t;
// pub fn isatty(fd: ::c_int) -> ::c_int;
// pub fn link(src: *const c_char, dst: *const c_char) -> ::c_int;
// pub fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t;
// pub fn pathconf(path: *const c_char, name: ::c_int) -> c_long;
// pub fn pipe(fds: *mut ::c_int) -> ::c_int;
// pub fn posix_memalign(memptr: *mut *mut ::c_void, align: ::size_t, size: ::size_t) -> ::c_int;

// pub fn rmdir(path: *const c_char) -> ::c_int;
// pub fn seteuid(uid: uid_t) -> ::c_int;
// pub fn setegid(gid: gid_t) -> ::c_int;
// pub fn setgid(gid: gid_t) -> ::c_int;
// pub fn setpgid(pid: pid_t, pgid: pid_t) -> ::c_int;
// pub fn setsid() -> pid_t;
// pub fn setuid(uid: uid_t) -> ::c_int;
// pub fn setreuid(ruid: uid_t, euid: uid_t) -> ::c_int;
// pub fn setregid(rgid: gid_t, egid: gid_t) -> ::c_int;
// pub fn sleep(secs: ::c_uint) -> ::c_uint;
// pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> ::c_int;
// pub fn tcgetpgrp(fd: ::c_int) -> pid_t;
// pub fn tcsetpgrp(fd: ::c_int, pgrp: ::pid_t) -> ::c_int;
// pub fn ttyname(fd: ::c_int) -> *mut c_char;
// pub fn ttyname_r(fd: ::c_int, buf: *mut c_char, buflen: ::size_t) -> ::c_int;
// pub fn unlink(c: *const c_char) -> ::c_int;
// pub fn wait(status: *mut ::c_int) -> pid_t;
// pub fn waitpid(pid: pid_t, status: *mut ::c_int, options: ::c_int) -> pid_t;
// pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
// pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
// pub fn umask(mask: mode_t) -> mode_t;
// pub fn utime(file: *const c_char, buf: *const utimbuf) -> ::c_int;

// pub fn kill(pid: pid_t, sig: ::c_int) -> ::c_int;
// pub fn killpg(pgrp: pid_t, sig: ::c_int) -> ::c_int;

// pub fn mlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
// pub fn munlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
// pub fn mlockall(flags: ::c_int) -> ::c_int;
// pub fn munlockall() -> ::c_int;
// pub fn mmap(addr: *mut ::c_void, len: ::size_t, prot: ::c_int, flags: ::c_int, fd: ::c_int, offset: off_t) -> *mut ::c_void;
// pub fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int;

// pub fn if_nametoindex(ifname: *const c_char) -> ::c_uint;
// pub fn if_indextoname(ifindex: ::c_uint, ifname: *mut ::c_char) -> *mut ::c_char;
// pub fn lstat(path: *const c_char, buf: *mut stat) -> ::c_int;

// pub fn fsync(fd: ::c_int) -> ::c_int;

// pub fn setenv(name: *const c_char, val: *const c_char, overwrite: ::c_int) -> ::c_int;
// pub fn unsetenv(name: *const c_char) -> ::c_int;

// pub fn symlink(path1: *const c_char, path2: *const c_char) -> ::c_int;

// pub fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int;

// pub fn signal(signum: ::c_int, handler: sighandler_t) -> sighandler_t;

// pub fn getrusage(resource: ::c_int, usage: *mut rusage) -> ::c_int;
// pub fn realpath(pathname: *const ::c_char, resolved: *mut ::c_char) -> *mut ::c_char;

// pub fn flock(fd: ::c_int, operation: ::c_int) -> ::c_int;

// pub fn times(buf: *mut ::tms) -> ::clock_t;
// pub fn getsockopt(sockfd: ::c_int, level: ::c_int, optname: ::c_int, optval: *mut ::c_void, optlen: *mut ::socklen_t) -> ::c_int;
// pub fn raise(signum: ::c_int) -> ::c_int;

// pub fn utimes(filename: *const ::c_char, times: *const ::timeval) -> ::c_int;
// pub fn dlopen(filename: *const ::c_char, flag: ::c_int) -> *mut ::c_void;
// pub fn dlerror() -> *mut ::c_char;
// pub fn dlsym(handle: *mut ::c_void, symbol: *const ::c_char) -> *mut ::c_void;
// pub fn dlclose(handle: *mut ::c_void) -> ::c_int;
// pub fn dladdr(addr: *const ::c_void, info: *mut Dl_info) -> ::c_int;
// pub fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> ::c_int;
// pub fn freeaddrinfo(res: *mut addrinfo);
// pub fn hstrerror(errcode: ::c_int) -> *const ::c_char;
// pub fn gai_strerror(errcode: ::c_int) -> *const ::c_char;

// pub fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
// pub fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
// pub fn mktime(tm: *mut tm) -> time_t;
// pub fn time(time: *mut time_t) -> time_t;
// pub fn gmtime(time_p: *const time_t) -> *mut tm;
// pub fn localtime(time_p: *const time_t) -> *mut tm;
// pub fn difftime(time1: time_t, time0: time_t) -> ::c_double;
// pub fn timegm(tm: *mut ::tm) -> time_t;
// pub fn mknod(pathname: *const ::c_char, mode: ::mode_t, dev: ::dev_t) -> ::c_int;
// pub fn gethostname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
// pub fn endservent();
// pub fn getservbyname(name: *const ::c_char, proto: *const ::c_char) -> *mut servent;
// pub fn getservbyport(port: ::c_int, proto: *const ::c_char) -> *mut servent;
// pub fn getservent() -> *mut servent;
// pub fn setservent(stayopen: ::c_int);
// pub fn getprotobyname(name: *const ::c_char) -> *mut protoent;
// pub fn getprotobynumber(proto: ::c_int) -> *mut protoent;
// pub fn chroot(name: *const ::c_char) -> ::c_int;
// pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t;
// pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t;
// pub fn putenv(string: *mut c_char) -> ::c_int;
// pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::c_int) -> ::c_int;
// pub fn select(nfds: ::c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> ::c_int;
// pub fn setlocale(category: ::c_int, locale: *const ::c_char) -> *mut ::c_char;
// pub fn localeconv() -> *mut lconv;

// pub fn sem_wait(sem: *mut sem_t) -> ::c_int;
// pub fn sem_trywait(sem: *mut sem_t) -> ::c_int;
// pub fn sem_post(sem: *mut sem_t) -> ::c_int;
// pub fn statvfs(path: *const c_char, buf: *mut statvfs) -> ::c_int;
// pub fn fstatvfs(fd: ::c_int, buf: *mut statvfs) -> ::c_int;

// pub fn sigemptyset(set: *mut sigset_t) -> ::c_int;
// pub fn sigaddset(set: *mut sigset_t, signum: ::c_int) -> ::c_int;
// pub fn sigfillset(set: *mut sigset_t) -> ::c_int;
// pub fn sigdelset(set: *mut sigset_t, signum: ::c_int) -> ::c_int;
// pub fn sigismember(set: *const sigset_t, signum: ::c_int) -> ::c_int;

// pub fn sigprocmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int;

// pub fn sigpending(set: *mut sigset_t) -> ::c_int;

// pub fn sysconf(name: ::c_int) -> ::c_long;

// pub fn mkfifo(path: *const c_char, mode: mode_t) -> ::c_int;

// pub fn fseeko(stream: *mut ::FILE, offset: ::off_t, whence: ::c_int) -> ::c_int;
// pub fn ftello(stream: *mut ::FILE) -> ::off_t;

// pub fn tcdrain(fd: ::c_int) -> ::c_int;
// pub fn cfgetispeed(termios: *const ::termios) -> ::speed_t;
// pub fn cfgetospeed(termios: *const ::termios) -> ::speed_t;
// pub fn cfsetispeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int;
// pub fn cfsetospeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int;
// pub fn tcgetattr(fd: ::c_int, termios: *mut ::termios) -> ::c_int;
// pub fn tcsetattr(fd: ::c_int, optional_actions: ::c_int, termios: *const ::termios) -> ::c_int;
// pub fn tcflow(fd: ::c_int, action: ::c_int) -> ::c_int;
// pub fn tcflush(fd: ::c_int, action: ::c_int) -> ::c_int;
// pub fn tcgetsid(fd: ::c_int) -> ::pid_t;
// pub fn tcsendbreak(fd: ::c_int, duration: ::c_int) -> ::c_int;
// pub fn mkstemp(template: *mut ::c_char) -> ::c_int;
// pub fn mkdtemp(template: *mut ::c_char) -> *mut ::c_char;
// pub fn tmpnam(ptr: *mut ::c_char) -> *mut ::c_char;
// pub fn openlog(ident: *const ::c_char, logopt: ::c_int, facility: ::c_int);
// pub fn closelog();
// pub fn setlogmask(maskpri: ::c_int) -> ::c_int;
// pub fn syslog(priority: ::c_int, message: *const ::c_char, ...);
// pub fn nice(incr: ::c_int) -> ::c_int;
// pub fn grantpt(fd: ::c_int) -> ::c_int;
// pub fn posix_openpt(flags: ::c_int) -> ::c_int;
// pub fn ptsname(fd: ::c_int) -> *mut ::c_char;
// pub fn unlockpt(fd: ::c_int) -> ::c_int;
// pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
// pub fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
// pub fn lockf(fd: ::c_int, cmd: ::c_int, len: ::off_t) -> ::c_int;
// pub fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> ::c_int;
// pub fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
// pub fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE;
// pub fn getsid(pid: pid_t) -> pid_t;
// pub fn truncate(path: *const c_char, length: off_t) -> ::c_int;
// pub fn pause() -> ::c_int;
// pub fn mkdirat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int;
// pub fn openat(dirfd: ::c_int, pathname: *const ::c_char, flags: ::c_int, ...) -> ::c_int;
// pub fn fdopendir(fd: ::c_int) -> *mut ::DIR;
// pub fn readdir_r(dirp: *mut ::DIR, entry: *mut ::dirent, result: *mut *mut ::dirent) -> ::c_int;
// pub fn readlinkat(dirfd: ::c_int, pathname: *const ::c_char, buf: *mut ::c_char, bufsiz: ::size_t) -> ::ssize_t;
// pub fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
// pub fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
// pub fn atexit(cb: extern "C" fn()) -> c_int;
// pub fn sigaction(signum: ::c_int, act: *const sigaction, oldact: *mut sigaction) -> ::c_int;
// pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: ::size_t) -> ::ssize_t;
// pub fn pselect(nfds: ::c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> ::c_int;
// pub fn cfmakeraw(termios: *mut ::termios);
// pub fn cfsetspeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int;