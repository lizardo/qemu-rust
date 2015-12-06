An example QEMU machine implemented in Rust. Requires applying the following changes to QEMU:

```diff
diff --git a/Makefile b/Makefile
index 930ac27..16612e8 100644
--- a/Makefile
+++ b/Makefile
@@ -228,6 +228,8 @@ libqemuutil.a: $(util-obj-y)
 block-modules = $(foreach o,$(block-obj-m),"$(basename $(subst /,-,$o))",) NULL
 util/module.o-cflags = -D'CONFIG_BLOCK_MODULES=$(block-modules)'
 
+util/module.o-cflags += -D'CONFIG_MACHINE_MODULES="machine-dummy", NULL'
+
 ######################################################################
 
 qemu-img.o: qemu-img-cmds.h
diff --git a/util/module.c b/util/module.c
index 4bd4a94..69bc254 100644
--- a/util/module.c
+++ b/util/module.c
@@ -171,6 +171,9 @@ static void module_load(module_init_type type)
     static const char *block_modules[] = {
         CONFIG_BLOCK_MODULES
     };
+    static const char *machine_modules[] = {
+        CONFIG_MACHINE_MODULES
+    };
     char *exec_dir;
     char *dirs[3];
     int i = 0;
@@ -185,6 +188,9 @@ static void module_load(module_init_type type)
     case MODULE_INIT_BLOCK:
         mp = block_modules;
         break;
+    case MODULE_INIT_MACHINE:
+        mp = machine_modules;
+        break;
     default:
         /* no other types have dynamic modules for now*/
         return;
```

Next, compile QEMU with module support:

```
./configure --enable-modules
```

Build this project:

```
cargo build
```

Finally, copy the shared object to the QEMU tree as "machine-dummy.so" (or
whatever module name you specified on the Makefile). Note that the list of
modules is static in QEMU and requires re-compilation in case of changes.

You may also need to change the `qemu_stamp_<id>()` function name to reflect
the stamp for your QEMU tree. This stamp can be found on the `config-host.mak`
file on the QEMU tree. Note that there is no ABI or API stability guarantee, so
this code may require changes for future QEMU versions.
